use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::io;
use std::fs::File;
use std::process::exit;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {

    use std::io::{BufRead};
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    println!("Welcome to the Répartiteur de coût.");

    // { ("montre", "corinne") : 80 }
    let mut gift_person_payment : HashMap<(String,String), f32> = HashMap::new();
    let mut gift_person_balance : HashMap<String, HashMap<String, f32>> = HashMap::new();
    let mut gift_cost : HashMap<String, f32> = HashMap::new();
    let mut final_person_balance : HashMap<String, f32> = HashMap::new();


    let mut predicats : Vec<String> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./predicats") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                predicats.push(ip.trim().to_string());
            }
        }
    }

    dbg!(&predicats);

    println!("**** Analyse *****");

    parse_predicate( &predicats,
                     &mut gift_person_payment,
                     &mut gift_person_balance,
                     &mut gift_cost);

    // dbg!(&gift_person_balance);


    println!("**** Repartition *****");

    init_repartition( &mut gift_person_balance,
                      &gift_cost);


    println!("**** Balance *****");

    compute_balance( &mut gift_person_balance,
                      &gift_person_payment);

    println!("Dette des personnes, par cadeau");
    dbg!(&gift_person_balance);

    println!("**** Montant dû par les personnes, négatif signifie que la persone doit de l'argent *****");

    compute_final_balance( &mut final_person_balance,
                     &gift_person_balance);

    dbg!(&final_person_balance);

}

/**
*/
fn compute_final_balance( final_person_balance : &mut HashMap<String, f32>,
                          gift_person_balance : &HashMap<String, HashMap<String, f32>>
) {
    for (_, person_soldes) in gift_person_balance {

        person_soldes.iter().for_each(|(person,v)| {

            if ! final_person_balance.contains_key(person) {
                // { "denis", -50 }
                final_person_balance.insert(person.to_string(), 0.0);
            }

            let total = final_person_balance.get_mut(person).unwrap();
            *total = *total + *v;
        });
    }

    let big_total : f32 = final_person_balance.values().into_iter().sum();

    // Rule 5
    if big_total.abs() > 0.3 {
        println!("💣 La somme des répartitions est erronées: {}", big_total);
    }
}

/**
*/
fn compute_balance( gift_person_balance : &mut HashMap<String, HashMap<String, f32>>,
                    gift_person_payment : &HashMap<(String,String), f32>
) {

    for ( (gift, person), price) in gift_person_payment {
        match gift_person_balance.get_mut(gift) {
            Some(pb) => {
                match pb.get_mut(person) {
                    Some(v) => *v = *v + *price,
                    None => {
                        pb.insert(person.to_string(),*price );
                    },
                }
            },
            None => println!("Missing entry for gift"),
        }

    }
}

/**
*/
fn init_repartition( gift_person_balance : &mut HashMap<String, HashMap<String, f32>>,
                    gift_cost : &HashMap<String, f32>
) {

    for (gift, price) in gift_cost {

        let n = match gift_person_balance.get(gift) {
            Some(s) => {
                s.len()
            },
            None => {
                // Rule 2
                println!("💣 La répartition du cadeau {} est manquante", &gift);
                exit(-78);
            },
        };


        let unitary_price : f32 = ( -1 as f32) * (*price / (n as f32));

        println!("Pour le cadeau [{}] chacun devra payer [{}]", gift, unitary_price);

        let pb = gift_person_balance.get_mut(gift).unwrap();

        pb.iter_mut().for_each(| ( _, v) | {
            *v = unitary_price;
        });
    }
}


/**
Verification rules
    R0 : Enlever les items blancs dans le vecteur de split
    R1 : 1 gift => 1 et uniquement 1 repartition
    R2 : 1 gift => 1 et uniquement 1 coût
    R3 : 1 gift et 1 personne => 0 ou 1 paiement
    R4 : 1 gift => total paiement égal au coût du gift
    R5 : Somme des répartitions finale équal à ZERO +/- eps
*/
fn parse_predicate( predicats : &Vec<String>,
                    gift_person_payment : &mut HashMap<(String,String), f32>,
                    gift_person_balance : &mut HashMap<String, HashMap<String, f32>>,
                    gift_cost : &mut HashMap<String, f32>
) {

    let mut list_gift : Vec<String> = vec![];

    // Analyse predicates
    for p in predicats {
        // Rule 0
        let parts = p.split(" ").into_iter().filter(|&item| item != "" ).collect::<Vec<&str>>();
        if p.contains("paye") {
            let person = parts.get(0).unwrap().deref();
            let _action = parts.get(1).unwrap().deref(); //
            let price = parts.get(2).unwrap().deref();
            let _pour = parts.get(3).unwrap().deref(); //
            let gift = parts.get(4).unwrap().deref();

            list_gift.push(gift.to_string());
            let key = (String::from(gift), String::from(person));
            // Rule 3
            if gift_person_payment.contains_key(&key) {
                println!("💣 Trop de paiement pour le cadeau {} et la personne {}", &key.0, &key.1);
                exit(-69);
            }

            gift_person_payment.insert(key, price.parse::<f32>().unwrap());

        } else if p.contains("coûte") {
            let gift = parts.get(0).unwrap().deref();
            let _action = parts.get(1).unwrap().deref();
            let price = parts.get(2).unwrap().deref();
            // Rule 2
            if gift_cost.contains_key(gift) {
                println!("💣 Trop de coûts pour le cadeau {}", &gift);
                exit(-79);
            }

            gift_cost.insert(gift.to_string(), price.parse::<f32>().unwrap());

        } else if p.contains( "repartition" ) {
            let gift = parts.get(0).unwrap().deref();
            let _action = parts.get(1).unwrap().deref();
            for i in 2..parts.len() {
                let person = parts.get(i).unwrap().deref();
                if person.trim() != "" {
                    if ! gift_person_balance.contains_key(gift) {
                        // { "montre", {} }
                        gift_person_balance.insert(gift.to_string(), HashMap::new() );
                    }

                    let pb = gift_person_balance.get_mut(gift ).unwrap();
                    // Rule 1
                    if pb.contains_key(person) {
                        println!("💣 Trop de répartition pour le cadeau {} et personne {}", &gift, &person);
                        exit(-99);
                    } else {
                        pb.insert(person.to_string(), 0.0);
                    }
                }
            }
        }

    } // end loop

    // verify the gift_cost is complete and payments are corrects
    check_healthy_payment(&list_gift, &gift_person_payment, &gift_cost);

}

/**
*/
fn check_healthy_payment(list_gift : &Vec<String>, gift_person_payment : &HashMap<(String,String), f32>, gift_cost: &HashMap<String, f32> ) {
    for g in list_gift {
        // Rule 1
        match gift_cost.get(g) {
            Some(cost) => {
                // Rule 4
                let mut total : f32 = 0.0;
                gift_person_payment.iter().for_each(|(k,paiement)| {
                    if k.0 == *g {
                        total = total + *paiement;
                    }
                });

                if *cost != total {
                    println!("💣 Mauvais paiement pour le cadeau {}, coût {}, paiement {}", &g, cost, total);
                    exit(-59);
                }
            },
            None => {
                println!("💣 Coût manquant pour cadeau {}", &g);
                exit(-68);
            }
        }
    }
}