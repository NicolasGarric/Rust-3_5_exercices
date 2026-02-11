use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    /*
    EXERCICES PRATIQUES — Rust Book Chapitres 2 et 3 (max d’éléments)
    Format : 3 exercices (facile / moyen / difficile)
    Règles :
    - Uniquement des consignes TODO en texte, copiables/collables.
    - Tout dans cette fonction main().
    - Aucun indice technique : pas de mots-clés suggérés, pas de squelette, pas d’aide sur la structure.
    - Données modifiées (pas celles du livre).
    */

    /*
    ============================================================
    FACILE — Entrée, types, calcul, affichage (chap 2 + 3.2 + 3.6)
    ============================================================
    TODO
    1) Affiche : "Saisis un entier :"
    2) Lis la saisie utilisateur.
    3) Si la saisie n’est pas un entier, affiche : "Entrée invalide" et termine.
    4) Sinon :
        a) affiche "pair" si le nombre est divisible par 2, sinon "impair"
        b) affiche "positif" si le nombre est > 0, "zero" si = 0, "negatif" si < 0
    5) Affiche ensuite : "double = X" (X = le nombre multiplié par 2)
    */

    fn exercice_one() {
        println!("");
        println!("--- Exercice one ---");
        println!("");
        println!("Saisis un entier:");

        let mut number = String::new();

        io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrée invalide");
                return;
            }
        };

        if number % 2 == 0 {
            println!("Pair");
        } else {
            println!("Impair");
        }

        if number < 0 {
            println!("Negative");
        } else if number > 0 {
            println!("Positive");
        } else {
            println!("Zero");
        }

        let x = number * 2;
        println!("Double X = {}", x);
        println!("");

    }

    // exercice_one();


    /*
    ============================================================
    MOYEN — Jeu de devinette + compteur + messages (chap 2 + 3.1 + 3.6 + 3.7)
    ============================================================
    TODO
    1) Le programme choisit un nombre secret entier dans l’intervalle [15, 120].
    2) Le programme demande à l’utilisateur de proposer un nombre.
    3) Le programme recommence tant que le secret n’est pas trouvé.
    4) À chaque saisie :
        a) incrémente un compteur de tentatives (y compris si la saisie est invalide)
        b) si la saisie n’est pas un entier, afficher : "Entrée invalide"
        c) si la saisie est un entier mais hors de [15, 120], afficher : "Hors intervalle"
        d) sinon comparer au secret :
            - trop petit : "C'est plus"
            - trop grand : "C'est moins"
            - égal : afficher "Trouvé en X tentatives" puis arrêter
    5) Quand c’est trouvé, afficher aussi : "Dernier essai: N"
    */

    fn exercice_two() {
        let secret_number = rand::thread_rng().gen_range(15..=120);
        let mut attemps: u32 = 0;

        println!("Secret number is {}", secret_number);

        loop {
            attemps += 1;
            println!("");
            println!("Propose a number");

            let mut guess = String::new();

            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Propose a valid number");
                    continue;
                }
            };

            println!("You choose {}", &guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("");
                    println!("You win!");
                    println!("Secret number was {}", &secret_number);
                    break;
                }
            }
        }

        println!("You found in {} attemps", attemps);
    }

    // exercice_two();


    /*
    ============================================================
    DIFFICILE — Session interactive multi-commandes (max chap 2 + 3)
    ============================================================
    TODO
    Construis un programme interactif qui tourne en continu jusqu’à arrêt explicite.

    Règles générales :
    1) Le programme affiche au démarrage : "Mode session"
    2) Ensuite, il lit des lignes utilisateur à répétition.
    3) Chaque ligne est soit une commande, soit invalide.
    4) Les commandes valides possibles sont exactement :

        A) "add X"
            - X est un entier (peut être négatif)
            - effet : ajouter X à une somme `sum`, et incrémenter un compteur `count_add`
            - afficher : "OK"

        B) "mul X"
            - X est un entier
            - effet : multiplier un total `product` par X, et incrémenter un compteur `count_mul`
            - afficher : "OK"
            - note : `product` doit avoir une valeur initiale cohérente pour une multiplication

        C) "stats"
            - effet : afficher exactement 4 lignes :
                "sum = S"
                "product = P"
                "add = A"
                "mul = M"
                (S,P,A,M = valeurs actuelles)

        D) "table N"
            - N est un entier entre 2 et 9 inclus
            - effet : afficher la table de multiplication de N de 1 à 10, une ligne par produit, au format :
                "N x i = R"
            - si N est hors [2,9] : afficher "Hors intervalle"

        E) "reset"
            - effet : remettre à zéro l’état de session (somme, product, compteurs) et afficher "OK"

        F) "quit"
            - effet : afficher "fin" puis arrêter le programme

    Gestion des erreurs (sans donner d’indice sur comment faire) :
    5) Si la ligne ne correspond à aucune commande valide, afficher "Commande invalide" et redemander.
    6) Si la commande attend un nombre mais que ce n’est pas un entier, afficher "Entrée invalide" et redemander.
    7) Les espaces en trop au début/fin de ligne ne doivent pas casser le programme.

    Exigences de sortie :
    8) Pour chaque commande reconnue, l’affichage doit correspondre exactement aux textes demandés.
    9) Aucun autre texte ne doit être affiché (sauf les prompts initiaux demandés).

    Objectif : reprendre un maximum d’éléments des chapitres 2 et 3 :
    - lecture utilisateur, conversions de types, conditions, répétition, fonctions éventuelles, calculs, état mutable, affichage.
    */
}
