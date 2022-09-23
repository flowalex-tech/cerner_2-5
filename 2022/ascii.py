from PIL import Image, ImageFont, ImageDraw

# cerner_2tothe5th_2022

ShowText = 'Cerner 2^5th 2022'

font = ImageFont.truetype('arialbd.ttf', 15) 
size = font.getsize(ShowText) -
image = Image.new('1', size, 1)  
draw = ImageDraw.Draw(image)
draw.text((0, 0), ShowText, font=font)
for rownum in range(size[1]): 
    line = []
    for colnum in range(size[0]):
        if image.getpixel((colnum, rownum)): line.append(' '),
        else: line.append('#'),
    print ''.join(line)
