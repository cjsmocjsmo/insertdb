#!/bin/sh

# for directories
# for -d in *; do
# 	for video in *.mkv; do
# 		ffmpeg -i "$video" "${video%.mkv}.mp4"
# 		rm "$video";
# 	done
# done

for img in *.bmp; do
	ffmpeg -i "$img" "${img%.bmp}.jpg"
	rm "$img";
done

