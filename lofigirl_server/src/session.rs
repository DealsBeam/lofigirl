use anyhow::Result;
use lofigirl_shared_common::config::{ConfigError, LastFMConfig, ListenBrainzConfig};
use sqlx::SqlitePool;
use uuid::Uuid;

struct TokenDB {
    pool: SqlitePool,
}

impl TokenDB {
    pub async fn new(filename: &str) -> Result<Self> {
        let token_db = TokenDB {
            pool: SqlitePool::connect(&std::env::var(filename)?).await?,
        };
        Ok(token_db)
    }

    pub async fn get_or_generate_token(
        &self,
        lastfm: Option<LastFMConfig>,
        listenbrainz: Option<ListenBrainzConfig>,
    ) -> Result<String> {
        (lastfm.is_some() && listenbrainz.is_some())
            .then(|| ())
            .ok_or(ConfigError::EmptyListeners)?;
        let mut conn = self.pool.acquire().await?;
        let lfm_id = if let Some(lastfm) = lastfm {
            Some(self.get_lastfm_id(&lastfm).await?)
        } else {
            None
        };
        let lb_id = if let Some(listenbrainz) = listenbrainz {
            Some(self.get_listenbrainz_id(&listenbrainz).await?)
        } else {
            None
        };

        let optional_token = sqlx::query!(
            r#"
                SELECT token FROM tokens WHERE lastfm_id = ?1 AND listenbrainz_id = ?2
            "#,
            lfm_id,
            lb_id
        )
        .fetch_optional(&mut conn)
        .await?;
        match optional_token {
            Some(rec) => Ok(rec.token),
            None => {
                let token = Uuid::new_v4();
                let token_str = token.to_hyphenated().to_string();
                let _id = sqlx::query!(
                    r#"
                        INSERT INTO tokens ( token, lastfm_id, listenbrainz_id )
                        VALUES ( ?1, ?2, ?3)
                    "#,
                    token_str,
                    lfm_id,
                    lb_id
                )
                .execute(&mut conn)
                .await?
                .rows_affected();
                Ok(token_str)
            }
        }
    }

    async fn get_lastfm_id(&self, lastfm: &LastFMConfig) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;
        let optional_id = sqlx::query!(
            r#"
                SELECT id FROM lastfm WHERE username = ?1
            "#,
            lastfm.username
        )
        .fetch_optional(&mut conn)
        .await?;
        match optional_id {
            Some(rec) => Ok(rec.id),
            None => {
                let id = sqlx::query!(
                    r#"
                        INSERT INTO lastfm (username, password, api_key, api_secret)
                        VALUES ( ?1, ?2, ?3, ?4)
                    "#,
                    lastfm.username,
                    lastfm.password,
                    lastfm.api_key,
                    lastfm.api_secret,
                )
                .execute(&mut conn)
                .await?
                .last_insert_rowid();
                Ok(id)
            }
        }
    }

    async fn get_listenbrainz_id(&self, listenbrainz: &ListenBrainzConfig) -> Result<i64> {
        let mut conn = self.pool.acquire().await?;
        let optional_id = sqlx::query!(
            r#"
                SELECT id FROM listenbrainz WHERE token = ?1
            "#,
            listenbrainz.token
        )
        .fetch_optional(&mut conn)
        .await?;
        match optional_id {
            Some(rec) => Ok(rec.id),
            None => {
                let id = sqlx::query!(
                    r#"
                        INSERT INTO listenbrainz ( token )
                        VALUES ( ?1 )
                    "#,
                    listenbrainz.token,
                )
                .execute(&mut conn)
                .await?
                .last_insert_rowid();
                Ok(id)
            }
        }
    }

    pub async fn get_info_from_token(
        &self,
        token_str: &str,
    ) -> Result<(Option<LastFMConfig>, Option<ListenBrainzConfig>)> {
        let mut conn = self.pool.acquire().await?;
        let rec = sqlx::query!(
            r#"
                SELECT lastfm_id, listenbrainz_id FROM tokens
                WHERE token = ?1
            "#,
            token_str
        )
        .fetch_one(&mut conn)
        .await?;
        let lb_config = if let Some(id) = rec.listenbrainz_id {
            Some(self.get_listenbrainz_config(id).await?)
        } else {
            None
        };
        let lfm_config = if let Some(id) = rec.lastfm_id {
            Some(self.get_lastfm_config(id).await?)
        } else {
            None
        };
        Ok((lfm_config, lb_config))
    }

    async fn get_listenbrainz_config(&self, id: i64) -> Result<ListenBrainzConfig> {
        let mut conn = self.pool.acquire().await?;
        let rec = sqlx::query!(
            r#"
                SELECT token FROM listenbrainz
                WHERE id = ?1
            "#,
            id
        )
        .fetch_one(&mut conn)
        .await?;
        Ok(ListenBrainzConfig { token: rec.token })
    }

    async fn get_lastfm_config(&self, id: i64) -> Result<LastFMConfig> {
        let mut conn = self.pool.acquire().await?;
        let rec = sqlx::query!(
            r#"
                SELECT username, password, api_key, api_secret FROM lastfm
                WHERE id = ?1
            "#,
            id
        )
        .fetch_one(&mut conn)
        .await?;
        Ok(LastFMConfig {
            api_key: rec.api_key,
            api_secret: rec.api_secret,
            username: rec.username,
            password: rec.password,
        })
    }
}
