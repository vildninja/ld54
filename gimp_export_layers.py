img = gimp.image_list()[0]
img_width = img.width
img_height = img.height
for layer in img.layers:
  if layer.name.startswith("."):
  	continue
  pdb.gimp_image_set_active_layer(img, layer)
  pdb.plug_in_autocrop_layer(img, layer)
  layer.resize(layer.width + 4, layer.height + 4, 2, 2)
  layer.name = layer.name.replace("Pasted Layer #", "house_")
  offsets = layer.offsets
  img.resize(layer.width, layer.height, offsets[0], offsets[1])
  path = "Documents/code/ldj54/res/" + layer.name + ".png"
  pdb.file_png_save_defaults(img, layer, path, path)
  path
  img.resize(img_width, img_height, -offsets[0], -offsets[1])

