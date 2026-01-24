# Windsurf Account Manager - Simple

Une application de bureau de gestion multi-comptes Windsurf développée avec Tauri + Vue 3 + TypeScript, conçue pour gérer plusieurs comptes Windsurf et offrir des fonctionnalités telles que la réinitialisation des crédits, la consultation de la facturation et le changement de compte en un clic.

> ⚠️ **Clause de non-responsabilité - Logiciel gratuit** : Ce logiciel est totalement gratuit. Si vous l'avez payé, vous vous êtes fait arnaquer !

## 📱 Groupes de discussion

<p align="center">
  <img src="../../public/交流群.png" alt="Code QR WeChat" width="300">
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../../public/QQ群.jpg" alt="Code QR QQ" width="300">
</p>

---

## 🖥️ Interface du logiciel

<p align="center">
  <img src="../../public/主页.png" alt="Accueil du logiciel" width="800">
</p>

---

## ✨ Fonctionnalités

### 🔐 Gestion des comptes
- ✅ **Ajouter/Modifier/Supprimer des comptes** - Opérations CRUD complètes pour les comptes.
- ✅ **Gestion des groupes** - Prise en charge des groupes personnalisés pour organiser facilement plusieurs comptes.
- ✅ **Système de tags** - Ajoutez des étiquettes personnalisées aux comptes.
- ✅ **Affichage du statut en temps réel** - Affiche le type de forfait, le solde des crédits, la date d'expiration, etc.
- ✅ **Opérations par lot** - Sélection multiple pour la réinitialisation ou la suppression en masse.
- ✅ **Stockage sécurisé** - Utilise le chiffrement AES-256-GCM avec des clés stockées dans le trousseau système.

### 💳 Réinitialisation des crédits
- ✅ **Réinitialisation en un clic** - Réinitialise les crédits via les API de mise à jour du nombre de sièges.
- ✅ **Rotation intelligente des sièges** - Alterne automatiquement entre 3, 4 et 5 sièges.
- ✅ **Réinitialisation par lot** - Prise en charge de la réinitialisation simultanée de plusieurs comptes (jusqu'à 5).
- ✅ **Réinitialisation d'équipe** - Réinitialisation en un clic pour tous les membres d'une équipe.
- ✅ **Planification automatique** - Définissez des tâches planifiées pour les réinitialisations automatiques.

### 👥 Gestion d'équipe
- ✅ **Voir les membres de l'équipe** - Liste toutes les informations des membres de l'équipe.
- ✅ **Inviter des membres** - Invitez de nouveaux membres via e-mail.
- ✅ **Supprimer des membres** - Retirez des membres spécifiques de l'équipe.
- ✅ **Gestion des quotas d'équipe** - Gestion unifiée des crédits des membres de l'équipe.

### 🔄 Changement de compte en un clic
- ✅ **Changement rapide** - Basculez rapidement vers d'autres comptes Windsurf.
- ✅ **Rafraîchissement automatique du Token** - Utilise automatiquement le `refresh_token` pour obtenir un nouvel `access_token`.
- ✅ **Déclencheur de rappel OAuth** - Finalise automatiquement la connexion via le protocole `windsurf://`.
- ✅ **Réinitialisation de l'ID machine** - Réinitialise l'identifiant de l'appareil (nécessite des privilèges d'administrateur).

### 🔧 Correctif de changement fluide (Seamless)
- ✅ **Détection auto du chemin Windsurf** - Trouve automatiquement l'emplacement d'installation.
- ✅ **Application du correctif en un clic** - Modifie `extension.js` pour une transition fluide.
- ✅ **Suppression de la limite de temps** - Supprime la restriction de délai OAuth de 180s.
- ✅ **Sauvegarde automatique** - Sauvegarde les fichiers originaux avant modification (jusqu'à 3 versions).
- ✅ **Restauration de l'état original** - Restauration en un clic à partir des fichiers de sauvegarde.
- ✅ **Redémarrage auto de Windsurf** - Redémarre automatiquement Windsurf après application du correctif.

### 💰 Paiements
- ✅ **Génération de cartes virtuelles** - Génère des informations de carte de crédit virtuelle pour tester les paiements.
- ✅ **BIN personnalisés** - Prise en charge de numéros BIN ou de plages BIN personnalisés.
- ✅ **Fenêtre de paiement privée** - Ouvre les pages Stripe dans une fenêtre de navigation privée indépendante.
- ✅ **Alipay/WeChat Pay** - Prise en charge des méthodes de paiement domestiques (Dons).

### 📊 Consultations de données
- ✅ **Informations de facturation** - Consultez le forfait, les quotas, l'utilisation, etc.
- ✅ **Statut de l'abonnement** - Affiche le type d'abonnement, la date d'expiration et la prochaine date de facturation.
- ✅ **Statistiques d'utilisation** - Affichez l'utilisation des crédits et le quota restant.
- ✅ **Rafraîchissement global** - Mise à jour en un clic de toutes les informations des comptes.

### ⚙️ Paramètres système
- ✅ **Configuration du proxy** - Prise en charge des paramètres de proxy HTTP.
- ✅ **Mode API léger** - Utilise `GetPlanStatus` à la place de `GetCurrentUser` pour réduire les requêtes.
- ✅ **Affichage détaillé des résultats** - Option pour afficher la réponse API détaillée.
- ✅ **Logs d'opérations** - Enregistre tout l'historique des opérations, exportable.

### 🔒 Sécurité des données
- ✅ **Trousseau système** - Stocke les clés de chiffrement dans le Gestionnaire d'identifiants Windows.
- ✅ **Chiffrement AES-256-GCM** - Toutes les informations sensibles sont chiffrées.
- ✅ **Stockage local** - Les données sont stockées uniquement localement.
- ✅ **Audit** - Dossiers complets pour l'audit des opérations.

## Stack Technique

### Frontend
- **Framework** : Vue 3 + TypeScript
- **Composants UI** : Element Plus
- **Gestion d'état** : Pinia
- **Outil de build** : Vite
- **Styles** : CSS3 + Thème Element Plus

### Backend
- **Framework** : Tauri 2.x
- **Langage** : Rust
- **Chiffrement** : AES-256-GCM
- **Gestion des clés** : Windows Credential Manager / Keyring
- **Requêtes réseau** : Reqwest
- **Runtime asynchrone** : Tokio

## Installation et Utilisation

### Prérequis
- Node.js 16+
- Rust 1.70+
- Windows 10/11 (Actuellement uniquement Windows)

### Environnement de développement

```bash
# Cloner le projet
git clone [repository-url]
cd windsurf-account-manager

# Installer les dépendances
npm install

# Lancer en mode développement
npm run tauri dev
```

### Build (Release)

```bash
# Créer l'installateur Windows
npm run tauri build
```

L'installateur se trouvera dans `src-tauri/target/release/bundle/` après le build.

## Guide d'utilisation

### 1. Première utilisation
1. Lancez l'app et cliquez sur "Ajouter un compte".
2. Entrez les infos : e-mail, mot de passe, nom, groupe, tags.
3. Cliquez sur OK pour sauvegarder.

### 2. Réinitialisation des crédits
1. Cliquez sur "Réinitialiser les crédits" sur une carte de compte.
2. L'app effectue la rotation des sièges (3, 4, 5).
3. Une notification confirmera le succès.

## Licence
AGPL-3.0

## Clause de non-responsabilité
Cet outil est destiné à un usage éducatif et personnel uniquement. Veuillez respecter les conditions d'utilisation de Windsurf. L'auteur n'est pas responsable des problèmes résultant de l'utilisation de cet outil.
