//
// Created by kavis on 20. 1. 14..
//

extern double fteff_ZARho_(double *Tb, double *Z, double *A, double *Rho, double *gs14);
extern double fteff_NT_(double *Tb, double *gs14);
extern double fteff_GPE_(double *Tb, double *gs14);
extern double fteff_acc_(double *Tb, double *eta, double *gs14);
extern double fteff_field_iron_(double *Tb, double *bfield, double *gs14);
extern void SPLINT1_(double *XA, double *YA, double *Y2A, int *IN, double *X, double *Y);
extern void SPLINE1_(double *X, double *Y, int *IN, double *YP1, double *YPN, double *Y2);