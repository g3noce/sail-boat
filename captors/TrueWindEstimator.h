#ifndef TRUE_WIND_ESTIMATOR_H
#define TRUE_WIND_ESTIMATOR_H

#include "arm_math.h"

// Structure de sortie
struct VentReel {
    float vitesse; // en m/s
    float cap;     // en degrés (0 à 360)
};

class TrueWindEstimator {
private:
    // Paramètres physiques fixes
    float hauteur_mat;
    
    // Variables pré-calculées pour la déclinaison magnétique
    float cos_dec;
    float sin_dec;

    // --- Matrices CMSIS-DSP pour le Filtre de Kalman 2D ---
    float x_data[2]; // État X : [Vent_Nord, Vent_Est]
    float p_data[4]; // Covariance P (2x2)
    float q_data[4]; // Bruit du modèle Q (2x2)
    float r_data[4]; // Bruit de la mesure R (2x2)
    float i_data[4]; // Matrice Identité I (2x2)

    arm_matrix_instance_f32 X;
    arm_matrix_instance_f32 P;
    arm_matrix_instance_f32 Q;
    arm_matrix_instance_f32 R;
    arm_matrix_instance_f32 I;

public:
    // Constructeur
    TrueWindEstimator();

    // Initialisation (à appeler dans le setup)
    void Init();

    // Fonction de mise à jour (à appeler à chaque tick de la girouette)
    void Update(float vent_vitesse_brut, float vent_angle_brut, 
                float gyro_x, float gyro_y, 
                float qw, float qx, float qy, float qz,
                float gps_vn, float gps_ve);

    // Récupération du résultat filtré
    VentReel GetFilteredWind();
};

#endif // TRUE_WIND_ESTIMATOR_H
