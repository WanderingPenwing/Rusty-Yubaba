import os
import numpy as np
import imageio.v2 as io
from scipy.ndimage import rotate
from PIL import Image

def symetrie(image):
    return image[:, ::-1]

def rotation(image, angle):
    rotated_image = rotate(image, angle, reshape=True, mode='constant', cval=0)
    return rotated_image
    

def negative(image):
    return 255 - image

def convertir_en_niveaux_de_gris(image):
    if image.ndim == 3 and image.shape[2] == 3:
        r, g, b = image[:,:,0], image[:,:,1], image[:,:,2]
        y = 0.2126 * r + 0.7152 * g + 0.0722 * b
        image_grise = np.round(y).astype(np.uint8)
        return image_grise
    elif image.ndim == 2:
        return image
    else:
        print("L'image n'est pas en couleur ou n'a pas le bon format.")
        return image

def convolution(image, kernel):
    if image.ndim != 3:
        print("Convolution pour les images en niveaux de gris n'est pas supportée.")
        return image

    output = np.zeros(image.shape, dtype=np.float32)
    for color in range(3):
        output[:, :, color] = np.clip(np.round(
            np.convolve(image[:, :, color], kernel, mode='same')), 0, 255)
    return output.astype(np.uint8)

def lissage(image):
    kernel = np.ones((3, 3), dtype=np.float32) / 9
    return convolution(image, kernel)

def contraste(image):
    kernel = np.array([[-1, -1, -1], [-1, 9, -1], [-1, -1, -1]], dtype=np.float32)
    return convolution(image, kernel)

def repoussage(image):
    kernel = np.array([[-2, -1, 0], [-1, 1, 1], [0, 1, 2]], dtype=np.float32)
    return convolution(image, kernel)

def traiter_comprimer_image(image_paths, effets_params, output_folder, output_extension, qualite=85):
    for image_path in image_paths :
        image = io.imread(image_path)

        for effet, params in effets_params.items():
            if effet == "symetrie":
                image = symetrie(image)
            elif effet == "rotation":
                angle = int(params) if params.isdigit() else 0
                image = rotation(image, angle)
            elif effet == "negative":
                image = negative(image)
            elif effet == "niveaux_de_gris":
                image = convertir_en_niveaux_de_gris(image)
            elif effet == "lissage":
                image = lissage(image)
            elif effet == "contraste":
                image = contraste(image)
            elif effet == "repoussage":
                image = repoussage(image)
            else:
                print(f"Effet {effet} non reconnu. L'effet sera ignoré.")

        nom_base = os.path.basename(image_path).split('.')[0]
        effets_nom = "_".join(effets_params.keys())
        output_file = os.path.join(output_folder, f"{nom_base}_{effets_nom}_compresse.{output_extension}")

        if not os.path.isdir(output_folder):
            os.makedirs(output_folder, exist_ok=True)

        image_traitee = Image.fromarray(image)
        image_traitee.save(output_file, format=output_extension.upper(), quality=qualite)
        print(f"L'image compressée a été sauvegardée sous : {output_file}")
        
if __name__ == '__main__':
    traiter_comprimer_image(['image.png'], {'symetrie': {},'rotation': '90'}, 'output', 'png', 80)
        
