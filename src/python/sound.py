import ffmpeg

class Sound_convert ():
    def __init__(self, output, input):
        self.output = output
        self.input = input
    
    def convert (self):
        ffmpeg.input(self.input).output(self.output).run()

if __name__ == '__main__':
    a = Sound_convert('', 'Macroblank.mp3')