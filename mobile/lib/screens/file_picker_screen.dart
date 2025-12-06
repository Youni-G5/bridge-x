import 'package:flutter/material.dart';
import 'package:file_picker/file_picker.dart';
import 'dart:io';
import '../services/api_service.dart';

class FilePickerScreen extends StatefulWidget {
  final String deviceId;
  final String deviceName;

  const FilePickerScreen({
    Key? key,
    required this.deviceId,
    required this.deviceName,
  }) : super(key: key);

  @override
  State<FilePickerScreen> createState() => _FilePickerScreenState();
}

class _FilePickerScreenState extends State<FilePickerScreen> {
  List<PlatformFile> selectedFiles = [];
  bool isUploading = false;
  double uploadProgress = 0.0;
  String? currentFileName;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Envoyer vers ${widget.deviceName}'),
        backgroundColor: Colors.blue.shade700,
      ),
      body: Column(
        children: [
          // Selected files list
          Expanded(
            child: selectedFiles.isEmpty
                ? Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.file_upload_outlined,
                          size: 80,
                          color: Colors.grey.shade400,
                        ),
                        const SizedBox(height: 20),
                        Text(
                          'Aucun fichier sélectionné',
                          style: TextStyle(
                            fontSize: 18,
                            color: Colors.grey.shade600,
                          ),
                        ),
                        const SizedBox(height: 10),
                        Text(
                          'Appuyez sur le bouton ci-dessous pour choisir',
                          style: TextStyle(
                            fontSize: 14,
                            color: Colors.grey.shade500,
                          ),
                        ),
                      ],
                    ),
                  )
                : ListView.builder(
                    itemCount: selectedFiles.length,
                    padding: const EdgeInsets.all(16),
                    itemBuilder: (context, index) {
                      final file = selectedFiles[index];
                      return Card(
                        child: ListTile(
                          leading: Icon(
                            _getFileIcon(file.extension ?? ''),
                            color: Colors.blue,
                            size: 32,
                          ),
                          title: Text(
                            file.name,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                          ),
                          subtitle: Text(
                            _formatFileSize(file.size),
                            style: TextStyle(color: Colors.grey.shade600),
                          ),
                          trailing: IconButton(
                            icon: const Icon(Icons.close, color: Colors.red),
                            onPressed: () {
                              setState(() {
                                selectedFiles.removeAt(index);
                              });
                            },
                          ),
                        ),
                      );
                    },
                  ),
          ),

          // Upload progress
          if (isUploading)
            Container(
              padding: const EdgeInsets.all(16),
              child: Column(
                children: [
                  LinearProgressIndicator(
                    value: uploadProgress,
                    backgroundColor: Colors.grey.shade300,
                    valueColor: const AlwaysStoppedAnimation<Color>(Colors.blue),
                  ),
                  const SizedBox(height: 10),
                  Text(
                    'Envoi de $currentFileName... ${(uploadProgress * 100).toStringAsFixed(0)}%',
                    style: const TextStyle(fontSize: 14),
                  ),
                ],
              ),
            ),

          // Action buttons
          Container(
            padding: const EdgeInsets.all(16),
            child: Row(
              children: [
                Expanded(
                  child: ElevatedButton.icon(
                    onPressed: isUploading ? null : _pickFiles,
                    icon: const Icon(Icons.add),
                    label: const Text('Ajouter des fichiers'),
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.symmetric(vertical: 14),
                      backgroundColor: Colors.blue.shade600,
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: ElevatedButton.icon(
                    onPressed: (selectedFiles.isEmpty || isUploading)
                        ? null
                        : _uploadFiles,
                    icon: const Icon(Icons.send),
                    label: const Text('Envoyer'),
                    style: ElevatedButton.styleFrom(
                      padding: const EdgeInsets.symmetric(vertical: 14),
                      backgroundColor: Colors.green.shade600,
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Future<void> _pickFiles() async {
    try {
      final result = await FilePicker.platform.pickFiles(
        allowMultiple: true,
        type: FileType.any,
      );

      if (result != null && result.files.isNotEmpty) {
        setState(() {
          selectedFiles.addAll(result.files);
        });
      }
    } catch (e) {
      _showError('Erreur lors de la sélection: $e');
    }
  }

  Future<void> _uploadFiles() async {
    setState(() {
      isUploading = true;
      uploadProgress = 0.0;
    });

    final apiService = ApiService();

    try {
      for (int i = 0; i < selectedFiles.length; i++) {
        final file = selectedFiles[i];
        
        setState(() {
          currentFileName = file.name;
          uploadProgress = i / selectedFiles.length;
        });

        if (file.path == null) {
          throw Exception('Chemin du fichier invalide');
        }

        await apiService.sendFile(
          deviceId: widget.deviceId,
          filePath: file.path!,
          onProgress: (progress) {
            setState(() {
              uploadProgress = (i + progress) / selectedFiles.length;
            });
          },
        );
      }

      // Success!
      if (mounted) {
        setState(() {
          isUploading = false;
          uploadProgress = 1.0;
        });

        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              '✅ ${selectedFiles.length} fichier(s) envoyé(s) avec succès !',
            ),
            backgroundColor: Colors.green,
          ),
        );

        // Clear and go back
        Future.delayed(const Duration(seconds: 2), () {
          if (mounted) {
            Navigator.pop(context);
          }
        });
      }
    } catch (e) {
      setState(() {
        isUploading = false;
      });
      _showError('Erreur lors de l\'envoi: $e');
    }
  }

  IconData _getFileIcon(String extension) {
    switch (extension.toLowerCase()) {
      case 'jpg':
      case 'jpeg':
      case 'png':
      case 'gif':
        return Icons.image;
      case 'pdf':
        return Icons.picture_as_pdf;
      case 'doc':
      case 'docx':
        return Icons.description;
      case 'mp4':
      case 'avi':
      case 'mkv':
        return Icons.video_file;
      case 'mp3':
      case 'wav':
        return Icons.audio_file;
      case 'zip':
      case 'rar':
        return Icons.folder_zip;
      default:
        return Icons.insert_drive_file;
    }
  }

  String _formatFileSize(int bytes) {
    if (bytes < 1024) return '$bytes B';
    if (bytes < 1024 * 1024) return '${(bytes / 1024).toStringAsFixed(1)} KB';
    if (bytes < 1024 * 1024 * 1024) {
      return '${(bytes / (1024 * 1024)).toStringAsFixed(1)} MB';
    }
    return '${(bytes / (1024 * 1024 * 1024)).toStringAsFixed(1)} GB';
  }

  void _showError(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: Colors.red,
        duration: const Duration(seconds: 4),
      ),
    );
  }
}
