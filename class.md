Client : 
    id 
    nom
    prenom 
    mail 
    num 
    date de naissance

Compte : 
    id_compte 
    client
    date de création 
    date de update 
    status 

Portefeuille : 
    id_portefeuille 
    montant
    liste_transactions

Transaction : 
    id_transaction
    idempotence_key
    type
    date_transaction