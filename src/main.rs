
extern crate rand;

use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::io;
use std::fs::File;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {

    use std::io::{BufRead};

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    println!("Welcome to Répartiteur de coût.");

    // { ("montre", "c") : 80 }
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

    /*
            Enlever les items blancs dans le vecteur de split

            TODO Verification rules
                1 gift => 1 et uniquement 1 repartition
                1 gift => 1 et uniquement 1 coût
                1 gift et 1 personne => 0 ou 1 paiement
                1 gift => total paiement égal au coût du gift
                Somme des répartitions finale équal à ZERO
     */

    println!("**** Analyse *****");

    parse_predicate( &predicats,
                     &mut gift_person_payment,
                     &mut gift_person_balance,
                     &mut gift_cost);

    // dbg!(&gift_person_payment);
    // dbg!(&gift_person_balance);
    // dbg!(&gift_cost);

    println!("**** Repartition *****");

    init_repartition( &mut gift_person_balance,
                      &gift_cost);

    // dbg!(&gift_person_balance);

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

fn compute_final_balance( final_person_balance : &mut HashMap<String, f32>,
                          gift_person_balance : &HashMap<String, HashMap<String, f32>>
) {

    for (_, person_soldes) in gift_person_balance {

        person_soldes.iter().for_each(|(person,v)| {

            if ! final_person_balance.contains_key(person) {
                // { "d", -50 }
                final_person_balance.insert(person.to_string(), 0.0);
            }

            let total = final_person_balance.get_mut(person).unwrap();
            *total = *total + *v;

        });
    }
}

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


fn init_repartition( gift_person_balance : &mut HashMap<String, HashMap<String, f32>>,
                    gift_cost : &HashMap<String, f32>
) {

    for (gift, price) in gift_cost {

        let n = gift_person_balance.get(gift).unwrap().len();
        let unitary_price : f32 = ( -1 as f32) * (*price / (n as f32));

        println!("Pour le cadeau [{}] chacun devra payer [{}]", gift, unitary_price);

        let pb = gift_person_balance.get_mut(gift).unwrap();

        pb.iter_mut().for_each(| ( _, v) | {
            *v = unitary_price;
        });
    }
}


/**
*/
fn parse_predicate( predicats : &Vec<String>,
                    gift_person_payment : &mut HashMap<(String,String), f32>,
                    gift_person_balance : &mut HashMap<String, HashMap<String, f32>>,
                    gift_cost : &mut HashMap<String, f32>
) {

    // Analyse predicates
    for p in predicats {
        // println!(">> {}", &p);
        let parts = p.split(" ").collect::<Vec<&str>>();
        if p.contains("paye") {
            let person = parts.get(0).unwrap().deref();
            let _action = parts.get(1).unwrap().deref(); //
            let price = parts.get(2).unwrap().deref();
            let _pour = parts.get(3).unwrap().deref(); //
            let gift = parts.get(4).unwrap().deref();
            //println!("PAY :  {} {} {} {} {}", &person, &action, &price, &pour, &gift);

            gift_person_payment.insert((String::from(gift), String::from(person)), price.parse::<f32>().unwrap_or(0.0));



        } else if p.contains("coûte") {
            let gift = parts.get(0).unwrap().deref();
            let _action = parts.get(1).unwrap().deref();
            let price = parts.get(2).unwrap().deref();
            //println!("COST :  {} {} {}",  &action, &price, &gift);

            gift_cost.insert(gift.to_string(), price.parse::<f32>().unwrap_or(0.0));

        } else if p.contains( "repartition" ) {
            let gift = parts.get(0).unwrap().deref();
            let _action = parts.get(1).unwrap().deref();
            // println!("SPLIT :  {} {}",  &action, &gift);
            for i in 2..parts.len() {
                let person = parts.get(i).unwrap().deref();
                if person.trim() != "" {
                    // println!("    PERSON :  [{}]", &person);
                    if ! gift_person_balance.contains_key(gift) {
                        // { "montre", {} }
                        gift_person_balance.insert(gift.to_string(), HashMap::new() );
                    }

                    let pb = gift_person_balance.get_mut(gift ).unwrap();
                    pb.insert(person.to_string(), 0.0);
                }
            }
        }

    }

}