import ffmpeg
import os

def audio_convert (input_list, output_folder, output_extension):
	for input_file in input_list :
		input_file = input_file.replace("\\", "/") #"
		input_name = input_file.split('.')
		
		input_path = input_name[0].split("/")
		
		if output_folder != "" and output_folder[-1] != "/" :
			output_folder += "/"

		output_file = output_folder + input_path[-1] + "." + output_extension
		i = 1
		while os.path.exists(output_file) :
			output_file = output_folder + input_path[-1] + "_" + str(i) + "." + output_extension
			i += 1

		print(input_file, output_file)

		ffmpeg.input(input_file).output(output_file).run()


if __name__ == '__main__':
	audio_convert(['Macroblank.mp3'], 'output', 'wav')
