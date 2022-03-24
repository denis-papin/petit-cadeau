
<style>
    .indent {
        margin-left:2.5rem;
        padding-left:0.5rem;
        border-left: blue solid 1px;
    }
</style>


### Répartiteur 

D'après une liste de règles simples, le programme va déterminer le montant que chaque personne doit.

** Exemple de règles **

<div class='indent'>
victor paye 3.00  pour wifi_ringo <br>
juju paye 82.36 pour wifi_ringo <br>
wifi_ringo repartition juju <br>
wifi_ringo  coûte 85.36 <br>
<br>
ringo paye 100 pour lampe_maria <br>
lampe_maria coûte 100 <br>
lampe_maria  repartition juju  ringo katie <br>
<br>
maria paye 39.05 pour assiette_maria <br>
juju paye  0.90 pour  assiette_maria <br>
assiette_maria coûte 39.95 <br>
assiette_maria repartition juju ringo victor <br>
</div>

** Règles ** 

Il existe trois sortes de règles :

```
{personne} paye {montant} pour {cadeau}
{cadeau} repartition {personne} {personne} ...
{cadeau} coûte {prix}
```
