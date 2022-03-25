time=$(date "+%Y%m%d-%H%M%S")

git add ./
if [ -z "$1" ];
then
    git commit -m "${time}"
else
    git commit -m "$1"
fi
git push
