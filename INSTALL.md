# 📥 Guide d'Installation BridgeX

> **Installation simple en 2 minutes** - Aucune connaissance technique requise !

## 🪟 Windows

### Téléchargement

1. **Allez sur la [page des releases](https://github.com/Youni-G5/bridge-x/releases/latest)**
2. **Téléchargez** `BridgeX-setup.exe` (environ 15 MB)
3. **Ouvrez** le fichier téléchargé

### Installation

1. L'installateur s'ouvre → Cliquez sur **"Next"** (Suivant)
2. Acceptez les conditions → Cliquez sur **"Install"** (Installer)
3. Windows peut demander une autorisation → Cliquez sur **"Oui"**
4. Attendez 10-15 secondes pendant l'installation
5. Cliquez sur **"Finish"** (Terminer)

### Premier lancement

1. **Ouvrez BridgeX** depuis le menu Démarrer ou le raccourci bureau
2. L'application démarre automatiquement le serveur en arrière-plan
3. Vous êtes prêt à utiliser BridgeX ! 🎉

> 💡 **Astuce** : Si Windows Defender bloque l'app, cliquez sur "Plus d'infos" puis "Exécuter quand même"

---

## 🍎 macOS

### Téléchargement

1. **Allez sur la [page des releases](https://github.com/Youni-G5/bridge-x/releases/latest)**
2. **Téléchargez** `BridgeX.dmg` (environ 20 MB)
3. **Ouvrez** le fichier DMG téléchargé

### Installation

1. Une fenêtre s'ouvre avec l'icône BridgeX
2. **Glissez l'icône BridgeX** vers le dossier **Applications**
3. Attendez la copie (5-10 secondes)
4. Éjectez le disque DMG

### Premier lancement

1. **Ouvrez BridgeX** depuis Applications ou Spotlight (⌘+Espace)
2. macOS peut afficher "BridgeX n'est pas vérifiée" :
   - Allez dans **Préférences Système** → **Sécurité**
   - Cliquez sur **"Ouvrir quand même"**
   - Confirmez en cliquant **"Ouvrir"**
3. L'application démarre automatiquement le serveur
4. Vous êtes prêt ! 🎉

> 💡 **Alternative** : Faites clic droit → "Ouvrir" sur l'icône BridgeX pour contourner la vérification

---

## 🐧 Linux

### Option 1 : AppImage (Recommandé)

**Compatible avec toutes les distributions Linux**

1. **Téléchargez** `BridgeX.AppImage` depuis [releases](https://github.com/Youni-G5/bridge-x/releases/latest)
2. **Ouvrez un terminal** dans le dossier de téléchargement
3. **Rendez le fichier exécutable** :
   ```bash
   chmod +x BridgeX-*.AppImage
   ```
4. **Lancez l'application** :
   ```bash
   ./BridgeX-*.AppImage
   ```
5. Vous êtes prêt ! 🎉

> 💡 **Conseil** : Déplacez l'AppImage dans `/opt` ou `~/Applications` pour l'organiser

### Option 2 : Paquet DEB (Ubuntu/Debian)

1. **Téléchargez** `bridgex.deb` depuis [releases](https://github.com/Youni-G5/bridge-x/releases/latest)
2. **Installez avec dpkg** :
   ```bash
   sudo dpkg -i bridgex-*.deb
   ```
3. Si des dépendances manquent :
   ```bash
   sudo apt-get install -f
   ```
4. **Lancez BridgeX** depuis le menu Applications ou :
   ```bash
   bridgex
   ```

---

## 📱 Android

### Téléchargement

1. **Téléchargez** `BridgeX.apk` depuis [releases](https://github.com/Youni-G5/bridge-x/releases/latest)
2. **Ouvrez le fichier APK** depuis vos téléchargements

### Installation

1. Android affiche "Installation bloquée" :
   - Cliquez sur **"Paramètres"**
   - Activez **"Autoriser de cette source"**
   - Revenez en arrière
2. Cliquez sur **"Installer"**
3. Attendez 5-10 secondes
4. Cliquez sur **"Ouvrir"**

### Premier lancement

1. L'app demande des permissions (stockage, caméra pour QR code)
2. Cliquez sur **"Autoriser"** pour chaque permission
3. Vous êtes prêt à scanner des QR codes ! 🎉

> 💡 **Alternative** : Installez via [F-Droid](https://f-droid.org) si disponible (plus sécurisé)

---

## 📱 iOS (À venir)

> ⚠️ L'application iOS est en cours de signature Apple. Elle sera disponible sur TestFlight prochainement.

**Pour être notifié** :
- ⭐ **Star** ce repo GitHub
- 👁️ **Watch** → Custom → Releases

---

## ✅ Vérifier que tout fonctionne

### Sur Desktop (Windows/macOS/Linux)

1. **Ouvrez BridgeX**
2. Dans la fenêtre principale, vous devriez voir :
   - ✅ "Backend server running" (serveur démarré)
   - ✅ Bouton "Pair Device" (Appairer un appareil)
   - ✅ Liste des appareils (vide au début)
3. Cliquez sur **"Pair Device"** → Un QR code s'affiche
4. ✅ **Tout fonctionne !**

### Sur Mobile (Android)

1. **Ouvrez BridgeX**
2. Vous devriez voir :
   - ✅ Bouton "Scan QR Code"
   - ✅ Liste des appareils (vide au début)
3. Cliquez sur **"Scan QR Code"** → La caméra s'ouvre
4. ✅ **Tout fonctionne !**

---

## 🔗 Connecter Desktop ↔ Mobile

**Pour transférer des fichiers, appairez vos appareils :**

1. **Sur Desktop** : Cliquez sur "Pair Device" → QR code s'affiche
2. **Sur Mobile** : Cliquez sur "Scan QR Code" → Pointez la caméra vers le QR
3. ✅ **Appareils connectés !**
4. Vous pouvez maintenant envoyer des fichiers dans les deux sens

---

## 🆘 Problèmes courants

### Windows : "Windows a protégé votre PC"

**Solution** :
1. Cliquez sur **"Informations complémentaires"**
2. Cliquez sur **"Exécuter quand même"**

### macOS : "Impossible d'ouvrir BridgeX"

**Solution** :
1. Allez dans **Préférences Système** → **Sécurité et confidentialité**
2. Cliquez sur **"Ouvrir quand même"**

### Linux : "Permission denied"

**Solution** :
```bash
chmod +x BridgeX-*.AppImage
```

### Android : "Installation bloquée"

**Solution** :
1. Paramètres → Sécurité → **Autoriser les sources inconnues**
2. Ou : Paramètres → Applications → Menu → Accès spécial → Installer des applications inconnues → Chrome → Autoriser

### Desktop : "Backend failed to start"

**Solution** :
1. Le port 8080 est peut-être utilisé par une autre app
2. Fermez les autres apps et relancez BridgeX
3. Ou changez le port dans les paramètres

### Mobile : "Cannot connect to desktop"

**Solution** :
1. Vérifiez que **desktop et mobile sont sur le même réseau WiFi**
2. Vérifiez que le firewall n'est pas bloqué (port 8080)
3. Réessayez le scan du QR code

---

## 🔧 Désinstallation

### Windows
1. Panneau de configuration → Programmes → Désinstaller un programme
2. Cherchez "BridgeX" → Clic droit → Désinstaller

### macOS
1. Ouvrez le dossier Applications
2. Glissez BridgeX vers la Corbeille
3. Videz la Corbeille

### Linux (AppImage)
1. Supprimez simplement le fichier `.AppImage`

### Linux (DEB)
```bash
sudo apt remove bridgex
```

### Android
1. Maintenez l'icône BridgeX appuyée
2. Cliquez sur "Désinstaller"

---

## 📚 Prochaines étapes

- ✅ Installation terminée ? → Lisez le [Guide d'utilisation](docs/USER_GUIDE.md)
- ⚙️ Configuration avancée ? → Voir [Configuration](docs/CONFIGURATION.md)
- 🐛 Problème non résolu ? → [Ouvrir une issue](https://github.com/Youni-G5/bridge-x/issues/new)
- 💬 Questions ? → [Discussions GitHub](https://github.com/Youni-G5/bridge-x/discussions)

---

## 🎉 Bienvenue dans BridgeX !

Merci d'avoir installé BridgeX. Si vous aimez l'app :
- ⭐ **Donnez une star** sur GitHub
- 🐦 **Partagez** avec vos amis
- 💡 **Proposez** des améliorations

**Fait avec ❤️ par la communauté BridgeX**
