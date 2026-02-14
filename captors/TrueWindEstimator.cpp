#include "TrueWindEstimator.h"
#include <cmath>

// Constructeur : Lie les matrices CMSIS-DSP à leurs tableaux de données en mémoire
TrueWindEstimator::TrueWindEstimator() {
    hauteur_mat = 0.6f; // Hauteur du mât en mètres

    arm_mat_init_f32(&X, 2, 1, x_data);
    arm_mat_init_f32(&P, 2, 2, p_data);
    arm_mat_init_f32(&Q, 2, 2, q_data);
    arm_mat_init_f32(&R, 2, 2, r_data);
    arm_mat_init_f32(&I, 2, 2, i_data);
}

// Initialisation : Remplissage des matrices avec les paramètres de réglage
void TrueWindEstimator::Init() {
    // Optimisation CPU : Pré-calcul de la déclinaison magnétique (-1.5° pour le Morbihan)
    float declinaison_rad = -1.5f * (float)M_PI / 180.0f;
    cos_dec = cosf(declinaison_rad);
    sin_dec = sinf(declinaison_rad);

    // État initial (Vent nul)
    x_data[0] = 0.0f;
    x_data[1] = 0.0f;

    // Matrice Identité
    i_data[0] = 1.0f; i_data[1] = 0.0f;
    i_data[2] = 0.0f; i_data[3] = 1.0f;

    // Covariance P initiale (Forte incertitude au démarrage)
    p_data[0] = 10.0f; p_data[1] = 0.0f;
    p_data[2] = 0.0f;  p_data[3] = 10.0f;

    // Matrice Q : Bruit du modèle (Inertie du vent, très faible variation tolérée)
    q_data[0] = 0.01f; q_data[1] = 0.0f;
    q_data[2] = 0.0f;  q_data[3] = 0.01f;

    // Matrice R : Confiance dans les capteurs mécaniques (+-5% d'erreur)
    r_data[0] = 2.0f; r_data[1] = 0.0f;
    r_data[2] = 0.0f; r_data[3] = 2.0f;
}

// Boucle principale de calcul du vent réel
void TrueWindEstimator::Update(float vent_vitesse_brut, float vent_angle_brut,
    float gyro_x, float gyro_y,
    float qw, float qx, float qy, float qz,
    float gps_vn, float gps_ve)
{
    // === 1. CINÉMATIQUE DU MÂT (Bras de levier 2D) ===
    float v_mat_x = -hauteur_mat * gyro_y;
    float v_mat_y = hauteur_mat * gyro_x;

    // === 2. VENT APPARENT COMPENSÉ (Repère Bateau) ===
    float vent_app_x = vent_vitesse_brut * cosf(vent_angle_brut) - v_mat_x;
    float vent_app_y = vent_vitesse_brut * sinf(vent_angle_brut) - v_mat_y;

    // === 3. MATRICE DE ROTATION TRONQUÉE (DCM optimisée pour Z=0) ===
    float dcm_00 = qw * qw + qx * qx - qy * qy - qz * qz;
    float dcm_01 = 2.0f * (qx * qy - qw * qz);

    float dcm_10 = 2.0f * (qx * qy + qw * qz);
    float dcm_11 = qw * qw - qx * qx + qy * qy - qz * qz;

    float vent_ned_x = dcm_00 * vent_app_x + dcm_01 * vent_app_y;
    float vent_ned_y = dcm_10 * vent_app_x + dcm_11 * vent_app_y;

    // === 4. DÉCLINAISON MAGNÉTIQUE (Pré-calculée) ===
    float vent_vrai_nord = vent_ned_x * cos_dec - vent_ned_y * sin_dec;
    float vent_vrai_est = vent_ned_x * sin_dec + vent_ned_y * cos_dec;

    // === 5. CALCUL DU VENT RÉEL (Mesure Z) ===
    float mesure_vent_reel_n = vent_vrai_nord + gps_vn;
    float mesure_vent_reel_e = vent_vrai_est + gps_ve;

    // === 6. FILTRAGE DE KALMAN (Matriciel CMSIS-DSP 2x2) ===

    // Étape A : Prédiction (P = P + Q)
    arm_mat_add_f32(&P, &Q, &P);

    // Étape B : Calcul de l'Innovation Y = Mesure - État_Prédit
    float y_data[2] = { mesure_vent_reel_n - x_data[0], mesure_vent_reel_e - x_data[1] };
    arm_matrix_instance_f32 Y = { 2, 1, y_data };

    // Étape C : Matrices temporaires pour le calcul du Gain
    arm_matrix_instance_f32 S, S_inv, K;
    float s_data[4], sinv_data[4], k_data[4];
    arm_mat_init_f32(&S, 2, 2, s_data);
    arm_mat_init_f32(&S_inv, 2, 2, sinv_data);
    arm_mat_init_f32(&K, 2, 2, k_data);

    // Étape D : Calcul de la Covariance de l'Innovation (S = P + R)
    arm_mat_add_f32(&P, &R, &S);

    // Étape E : Calcul du Gain de Kalman (K = P * S_inv)
    arm_mat_inverse_f32(&S, &S_inv);
    arm_mat_mult_f32(&P, &S_inv, &K);

    // Étape F : Mise à jour de l'État (X = X + K * Y)
    arm_matrix_instance_f32 KY;
    float ky_data[2];
    arm_mat_init_f32(&KY, 2, 1, ky_data);

    arm_mat_mult_f32(&K, &Y, &KY);
    arm_mat_add_f32(&X, &KY, &X); // Met à jour x_data[0] et x_data[1]

    // Étape G : Mise à jour de la Covariance (P = (I - K) * P)
    arm_matrix_instance_f32 I_minus_K, TempP;
    float imk_data[4], tempp_data[4];
    arm_mat_init_f32(&I_minus_K, 2, 2, imk_data);
    arm_mat_init_f32(&TempP, 2, 2, tempp_data);

    arm_mat_sub_f32(&I, &K, &I_minus_K);
    arm_mat_mult_f32(&I_minus_K, &P, &TempP);

    // Copie du résultat sécurisée dans P
    p_data[0] = tempp_data[0]; p_data[1] = tempp_data[1];
    p_data[2] = tempp_data[2]; p_data[3] = tempp_data[3];
}

// Formatage de la sortie pour l'utilisateur
VentReel TrueWindEstimator::GetFilteredWind() {
    VentReel resultat;

    // Vitesse via Pythagore
    resultat.vitesse = sqrtf(x_data[0] * x_data[0] + x_data[1] * x_data[1]);

    // Cap via Arc-Tangente (conversion de Radians en Degrés)
    resultat.cap = atan2f(x_data[1], x_data[0]) * (180.0f / (float)M_PI);

    // Normalisation du cap entre 0 et 360°
    if (resultat.cap < 0.0f) {
        resultat.cap += 360.0f;
    }

    return resultat;
}