import 'package:flutter/material.dart';

class ServerSettings extends StatelessWidget {
  String? serverUrl;
  Function(String) onServerUrlChanged;
  ServerSettings(this.serverUrl, this.onServerUrlChanged);

  @override
  Widget build(BuildContext context) {
    return Column(children: [
      TextField(
        decoration: const InputDecoration(
          labelText: 'Server URL',
        ),
        onSubmitted: onServerUrlChanged,
        controller: TextEditingController(text: serverUrl),
      ),
    ]);
  }
}

class ListenBrainzSettings extends StatelessWidget {
  String? listenBrainzToken;
  Function(String) onListenBrainzTokenChanged;
  ListenBrainzSettings(this.listenBrainzToken, this.onListenBrainzTokenChanged);

  @override
  Widget build(BuildContext context) {
    return Column(children: [
      TextField(
        decoration: const InputDecoration(
          labelText: 'ListenBrainz Token',
        ),
        onSubmitted: onListenBrainzTokenChanged,
        controller: TextEditingController(text: listenBrainzToken),
      )
    ]);
  }
}

class LastFmSettings extends StatelessWidget {
  String? lastFmUsername;
  String? lastFMSessionKey;
  Function(String) onLastFMUsernameChanged;
  Function(String) onLastFMPasswordChanged;
  Function() onLastFMSessionKeyDeleted;
  LastFmSettings(
      this.lastFmUsername,
      this.lastFMSessionKey,
      this.onLastFMUsernameChanged,
      this.onLastFMPasswordChanged,
      this.onLastFMSessionKeyDeleted);

  @override
  Widget build(BuildContext context) {
    return Column(
        children: (lastFMSessionKey == null)
            ? [
                TextField(
                  decoration: const InputDecoration(
                    labelText: 'LastFM Username',
                  ),
                  onSubmitted: onLastFMUsernameChanged,
                  controller: TextEditingController(text: lastFmUsername),
                ),
                TextField(
                  decoration: const InputDecoration(
                    labelText: 'LastFM Password',
                  ),
                  obscureText: true,
                  onSubmitted: onLastFMPasswordChanged,
                  controller: TextEditingController(text: ''),
                )
              ]
            : [
                TextField(
                  decoration: const InputDecoration(
                    labelText: 'LastFM Username',
                  ),
                  onSubmitted: onLastFMUsernameChanged,
                  readOnly: true,
                  controller: TextEditingController(text: lastFmUsername),
                ),
                TextField(
                  decoration: const InputDecoration(
                    labelText: 'LastFM Session Key',
                  ),
                  readOnly: true,
                  controller: TextEditingController(text: lastFMSessionKey),
                ),
                ElevatedButton(
                  child: const Text('Delete Session Key'),
                  onPressed: onLastFMSessionKeyDeleted,
                )
              ]);
  }
}
