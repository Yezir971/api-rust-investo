# ADR 0001 : Choix du langage back
**Statut :** Accepté
**Date :** 2026-05-07
**Auteurs :** Charly, James

## Contexte
Notre projet InvesTo impose une contrainte :
- latence ultra faible au niveau mondial
Nous devons donc choisir un langage optimisé, et rapide, qui permette de faire des transactions assez vite. 
Sur le long terme, nous souhaitons ajouter de l'ia sur la recherche d'information
## Options envisagées
1. **Rust** : langage très rapide, possible d'intégrer de l'ia, haute sécurité mais coût de maintenance élevé.
2. **Go** : langage rapide, coût de maintenance faible, haute sécurité de code mais difficile d'y intégrer de l'ia.

## Décision
Nous décidons de partir sur Rust car sur une vision long terme on se projette mieux, notamment sur l'aspect d'intégration de l'ia et également car l'argent n'est pas réellement un problème pour le moment

## Conséquences
- **Positives :** quasiment aucune latence, forte securité
- **Négatives :** Pas facile de trouver des codeur rust, le prix est élevé
- **Risques :** trouver de moins en moins de développeur rust, le projet peut également prendre un certains temps avec se choix de langage
- **Réversibilité :** ~1 année pour recoder en go avec notre équipe de développeur.
