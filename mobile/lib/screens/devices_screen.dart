import 'package:flutter/material.dart';
import '../services/api_service.dart';
import 'file_picker_screen.dart';

class DevicesScreen extends StatefulWidget {
  const DevicesScreen({Key? key}) : super(key: key);

  @override
  State<DevicesScreen> createState() => _DevicesScreenState();
}

class _DevicesScreenState extends State<DevicesScreen> {
  final _apiService = ApiService();
  List<Map<String, dynamic>> _devices = [];
  bool _isLoading = true;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _loadDevices();
  }

  Future<void> _loadDevices() async {
    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });

    try {
      final devices = await _apiService.getDevices();
      setState(() {
        _devices = devices;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _errorMessage = e.toString();
        _isLoading = false;
      });
    }
  }

  void _sendFileTo(Map<String, dynamic> device) {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => FilePickerScreen(
          deviceId: device['id'] ?? '',
          deviceName: device['name'] ?? 'Unknown Device',
        ),
      ),
    );
  }

  Future<void> _deleteDevice(String deviceId) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Supprimer l\'appareil ?'),
        content: const Text(
          'Cet appareil sera déconnecté et devra \u00eatre reappairé.',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('Annuler'),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text(
              'Supprimer',
              style: TextStyle(color: Colors.red),
            ),
          ),
        ],
      ),
    );

    if (confirmed == true) {
      try {
        await _apiService.unpairDevice(deviceId);
        _loadDevices();
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Appareil supprimé')),
          );
        }
      } catch (e) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text('Erreur: $e'),
              backgroundColor: Colors.red,
            ),
          );
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Mes appareils'),
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: _loadDevices,
          ),
        ],
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : _errorMessage != null
              ? Center(
                  child: Padding(
                    padding: const EdgeInsets.all(24.0),
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.error_outline,
                          size: 64,
                          color: Colors.red.shade300,
                        ),
                        const SizedBox(height: 16),
                        Text(
                          _errorMessage!,
                          textAlign: TextAlign.center,
                          style: const TextStyle(color: Colors.red),
                        ),
                        const SizedBox(height: 24),
                        ElevatedButton(
                          onPressed: _loadDevices,
                          child: const Text('Réessayer'),
                        ),
                      ],
                    ),
                  ),
                )
              : _devices.isEmpty
                  ? Center(
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        children: [
                          Icon(
                            Icons.devices_other,
                            size: 80,
                            color: Colors.grey.shade400,
                          ),
                          const SizedBox(height: 20),
                          Text(
                            'Aucun appareil appairé',
                            style: TextStyle(
                              fontSize: 18,
                              color: Colors.grey.shade600,
                            ),
                          ),
                          const SizedBox(height: 10),
                          Text(
                            'Scannez un QR code pour commencer',
                            style: TextStyle(
                              fontSize: 14,
                              color: Colors.grey.shade500,
                            ),
                          ),
                        ],
                      ),
                    )
                  : ListView.builder(
                      itemCount: _devices.length,
                      padding: const EdgeInsets.all(16),
                      itemBuilder: (context, index) {
                        final device = _devices[index];
                        final deviceName = device['name'] ?? 'Unknown Device';
                        final deviceType = device['type'] ?? 'desktop';

                        return Card(
                          margin: const EdgeInsets.only(bottom: 12),
                          child: ListTile(
                            leading: CircleAvatar(
                              backgroundColor:
                                  Theme.of(context).primaryColor.withOpacity(0.1),
                              child: Icon(
                                deviceType == 'mobile'
                                    ? Icons.phone_android
                                    : Icons.computer,
                                color: Theme.of(context).primaryColor,
                              ),
                            ),
                            title: Text(deviceName),
                            subtitle: Text(deviceType),
                            trailing: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                IconButton(
                                  icon: const Icon(Icons.send, color: Colors.blue),
                                  onPressed: () => _sendFileTo(device),
                                  tooltip: 'Envoyer fichier',
                                ),
                                IconButton(
                                  icon: const Icon(Icons.delete, color: Colors.red),
                                  onPressed: () =>
                                      _deleteDevice(device['id'] ?? ''),
                                  tooltip: 'Supprimer',
                                ),
                              ],
                            ),
                          ),
                        );
                      },
                    ),
    );
  }
}
