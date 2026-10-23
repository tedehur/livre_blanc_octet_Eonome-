use octet_econome::{encode_ascii_compact, injecter_attaque, verificateur_noeud_embarque, StatutSysteme};

#[test]
fn flux_nominal_reste_nominal() {
    let message = "OK ";
    let flux = encode_ascii_compact(message);
    let verdict = verificateur_noeud_embarque(&flux, &flux, 20);

    assert!(matches!(verdict.statut, StatutSysteme::Nominal));
    assert_eq!(verdict.donnees_decodees, "OK ");
}

#[test]
fn divergence_locale_est_detectee() {
    let message = "DOSSIER PATIENT OK ";
    let mut flux = encode_ascii_compact(message);
    let reference = flux.clone();

    injecter_attaque(&mut flux, 60, 12);

    let verdict = verificateur_noeud_embarque(&flux, &reference, 64);

    assert!(matches!(verdict.statut, StatutSysteme::CritiqueAttaqueDetectee { .. }));
    assert!(verdict.bits_traites > 0);
}

#[test]
fn troncature_de_flux_est_detectee() {
    let message = "DOSSIER PATIENT OK ";
    let mut flux = encode_ascii_compact(message);
    let reference = flux.clone();

    // Truncature du flux principal : on coupe la fin du signal.
    flux.truncate(flux.len() / 2);

    let verdict = verificateur_noeud_embarque(&flux, &reference, 64);
    assert!(matches!(verdict.statut, StatutSysteme::CritiqueAttaqueDetectee { .. }));
}

#[test]
fn insertion_de_bits_est_detectee() {
    let message = "DOSSIER PATIENT OK ";
    let mut flux = encode_ascii_compact(message);
    let reference = flux.clone();

    // Insertion d’un bloc parasite au milieu du signal.
    flux.insert(20, false);
    flux.insert(21, true);
    flux.insert(22, false);

    let verdict = verificateur_noeud_embarque(&flux, &reference, 64);
    assert!(matches!(verdict.statut, StatutSysteme::CritiqueAttaqueDetectee { .. }));
}

#[test]
fn flux_vide_ne_trigger_pas_de_faux_positif() {
    let flux = Vec::<bool>::new();
    let verdict = verificateur_noeud_embarque(&flux, &flux, 10);

    assert!(matches!(verdict.statut, StatutSysteme::Nominal));
    assert_eq!(verdict.donnees_decodees, "");
}
