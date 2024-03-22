#import ffmpeg
from PIL import Image

TYPE = {
    'IMAGE' : ['jpeg','jpg','png','webp'],
	'VIDEO' : ['mp4'],
	'AUDIO' : ['mp3','wav']
}

def find_file_name(input_path):
	name = ''
	ext = ''
	parent = ''

	dot = False
	slash = False
	for _ in range(-1,-len(input_path)-1,-1):
		L = input_path[_]
		if L == '.' and not dot:
			dot = True
			continue
		if L == '/':
			slash = True
		
		if not dot:
			ext += L
		elif dot and not slash:
			name += L
		elif slash:
			parent += L
	return parent [::-1], name[::-1], ext[::-1]


def convert(input_path, ex_to=str,  output_path='') :
	file_type = None
	parent, file, ex_from = find_file_name(input_path)
	
	if output_path == '':
		output_path = parent + file + "." + ex_to
	else: 
		output_path += file + "." + ex_to

	for KEY in TYPE:
		for ext in TYPE[KEY]:
			if ext == ex_from:
				print('You are trying to convert a '+ KEY)
				file_conv = KEY
				break
		if file_conv != None:
			break
	
	if file_conv == 'IMAGE':
		(
			Image
			.open(input_path)
			.convert("RGB")
			.save(output_path, ex_to)
		)
	
	elif file_conv == 'VIDEO':
		pass
	elif file_conv == 'AUDIO':
		pass
	else:
		print("The file, you are trying to convert, is not supported")

if __name__ == '__main__':
	print(find_file_name("etc/test/turtle.webp"))
