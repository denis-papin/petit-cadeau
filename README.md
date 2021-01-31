### Répartiteur 

D'après une liste de règles simples, le programme va déterminer le montant que chaque personne doit.

**Exemple de règle**

```
victor paye 3.00  pour wifi_ringo
juju paye 82.36 pour wifi_ringo
wifi_ringo repartition juju
wifi_ringo  coûte 85.36

ringo paye 100 pour lampe_maria
lampe_maria coûte 100
lampe_maria  repartition juju  ringo katie

maria paye 39.05 pour assiette_maria
juju paye  0.90 pour  assiette_maria
assiette_maria coûte 39.95
assiette_maria repartition juju ringo victor
```

**Regles** 

Il existe trois sorte de règle :

```
{personne} paye {montant} pour {cadeau}
{cadeau} repartition {personne} {personne} ...
{cadeau} coûte {prix}
```