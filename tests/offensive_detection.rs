use octet_econome::{encode_ascii_compact, injecter_attaque, verificateur_noeud_embarque, StatutSysteme};

#[test]
fn encode_decode_nominal_flux_is_valid() {
    let message = "OK ";
    let flux = encode_ascii_compact(message);
    let verdict = verificateur_noeud_embarque(&flux, &flux, 20);

    assert!(matches!(verdict.statut, StatutSysteme::Nominal));
    assert_eq!(verdict.donnees_decodees, "OK ");
}

#[test]
fn divergence_on_alpha_is_detected() {
    let message = "DOSSIER PATIENT OK ";
    let mut flux = encode_ascii_compact(message);
    let reference = flux.clone();

    injecter_attaque(&mut flux, 60, 12);

    let verdict = verificateur_noeud_embarque(&flux, &reference, 64);

    assert!(matches!(verdict.statut, StatutSysteme::CritiqueAttaqueDetectee { .. }));
    assert!(verdict.bits_traites > 0);
}

#[test]
fn edge_case_empty_input_stays_nominal() {
    let flux = vec![];
    let verdict = verificateur_noeud_embarque(&flux, &flux, 10);

    assert!(matches!(verdict.statut, StatutSysteme::Nominal));
    assert_eq!(verdict.donnees_decodees, "");
    assert_eq!(verdict.bits_traites, 0);
}
