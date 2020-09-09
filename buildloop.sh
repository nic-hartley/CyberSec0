while inotifywait -e modify -e move -e delete -e create -r --exclude .git\|target\|docs .
do
  cargo run
done
