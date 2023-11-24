


def enleve_zeros(vecteur):
    for index, valeur in enumerate(vecteur):
        if valeur == 0:
            vecteur.pop(index)

