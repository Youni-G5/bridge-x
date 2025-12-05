import 'dart:io';
import 'package:flutter/material.dart';
import 'package:file_picker/file_picker.dart';
import 'package:provider/provider.dart';
import '../services/transfer_service.dart';
import '../models/device.dart';

class FilePickerScreen extends StatefulWidget {
  final Device device;

  const FilePickerScreen({super.key, required this.device});

  @override
  State<FilePickerScreen> createState() => _FilePickerScreenState();
}

class _FilePickerScreenState extends State<FilePickerScreen> {
  List<PlatformFile> _selectedFiles = [];
  bool _isUploading = false;
  double _uploadProgress = 0.0;
  String? _uploadStatus;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Send to ${widget.device.name}'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Device info card
            Card(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Row(
                  children: [
                    const Icon(Icons.computer, size: 40),
                    const SizedBox(width: 16),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            widget.device.name,
                            style: Theme.of(context).textTheme.titleLarge,
                          ),
                          Text(
                            'ID: ${widget.device.id.substring(0, 8)}...',
                            style: Theme.of(context).textTheme.bodySmall,
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: 24),

            // Select files button
            ElevatedButton.icon(
              onPressed: _isUploading ? null : _pickFiles,
              icon: const Icon(Icons.file_present),
              label: const Text('Select Files'),
              style: ElevatedButton.styleFrom(
                padding: const EdgeInsets.all(16),
              ),
            ),
            const SizedBox(height: 16),

            // Selected files list
            if (_selectedFiles.isNotEmpty) ..[
              Text(
                'Selected Files (${_selectedFiles.length})',
                style: Theme.of(context).textTheme.titleMedium,
              ),
              const SizedBox(height: 8),
              Expanded(
                child: ListView.builder(
                  itemCount: _selectedFiles.length,
                  itemBuilder: (context, index) {
                    final file = _selectedFiles[index];
                    return Card(
                      child: ListTile(
                        leading: const Icon(Icons.insert_drive_file),
                        title: Text(file.name),
                        subtitle: Text(_formatBytes(file.size)),
                        trailing: IconButton(
                          icon: const Icon(Icons.close),
                          onPressed: _isUploading
                              ? null
                              : () => _removeFile(index),
                        ),
                      ),
                    );
                  },
                ),
              ),
              const SizedBox(height: 16),
            ] else
              const Expanded(
                child: Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(
                        Icons.cloud_upload,
                        size: 80,
                        color: Colors.grey,
                      ),
                      SizedBox(height: 16),
                      Text(
                        'No files selected',
                        style: TextStyle(color: Colors.grey),
                      ),
                    ],
                  ),
                ),
              ),

            // Upload progress
            if (_isUploading) ..[
              LinearProgressIndicator(value: _uploadProgress),
              const SizedBox(height: 8),
              Text(
                _uploadStatus ?? 'Uploading...',
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 16),
            ],

            // Send button
            ElevatedButton.icon(
              onPressed: (_selectedFiles.isNotEmpty && !_isUploading)
                  ? _uploadFiles
                  : null,
              icon: const Icon(Icons.send),
              label: Text(
                _isUploading ? 'Uploading...' : 'Send Files',
              ),
              style: ElevatedButton.styleFrom(
                padding: const EdgeInsets.all(16),
                backgroundColor: Colors.blue,
                foregroundColor: Colors.white,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<void> _pickFiles() async {
    try {
      final result = await FilePicker.platform.pickFiles(
        allowMultiple: true,
        type: FileType.any,
      );

      if (result != null) {
        setState(() {
          _selectedFiles = result.files;
        });
      }
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error picking files: $e')),
      );
    }
  }

  void _removeFile(int index) {
    setState(() {
      _selectedFiles.removeAt(index);
    });
  }

  Future<void> _uploadFiles() async {
    setState(() {
      _isUploading = true;
      _uploadProgress = 0.0;
      _uploadStatus = 'Preparing upload...';
    });

    try {
      final transferService = context.read<TransferService>();

      for (var i = 0; i < _selectedFiles.length; i++) {
        final file = _selectedFiles[i];

        setState(() {
          _uploadStatus = 'Uploading ${file.name} (${i + 1}/${_selectedFiles.length})';
        });

        if (file.path != null) {
          await transferService.uploadFile(
            deviceId: widget.device.id,
            filePath: file.path!,
            onProgress: (progress) {
              setState(() {
                _uploadProgress = (i + progress) / _selectedFiles.length;
              });
            },
          );
        }
      }

      if (!mounted) return;

      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('✅ Files uploaded successfully!'),
          backgroundColor: Colors.green,
        ),
      );

      Navigator.pop(context);
    } catch (e) {
      if (!mounted) return;

      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('❌ Upload failed: $e'),
          backgroundColor: Colors.red,
        ),
      );
    } finally {
      setState(() {
        _isUploading = false;
        _uploadProgress = 0.0;
        _uploadStatus = null;
      });
    }
  }

  String _formatBytes(int bytes) {
    if (bytes < 1024) return '$bytes B';
    if (bytes < 1024 * 1024) return '${(bytes / 1024).toStringAsFixed(1)} KB';
    if (bytes < 1024 * 1024 * 1024) {
      return '${(bytes / (1024 * 1024)).toStringAsFixed(1)} MB';
    }
    return '${(bytes / (1024 * 1024 * 1024)).toStringAsFixed(1)} GB';
  }
}
