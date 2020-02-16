# set title "Comparison between spin loop and sleep, focusing on few threads"

set terminal pngcairo size 1200,700 enhanced font 'Inter,12'
set output '1000-threads-20ms-both-zoom-in.png'

set style line 101 lc rgb '#838383' lt 1 lw 1
set border 3 front ls 101
set tics nomirror out scale 0.75
unset key

set ylabel "Wall clock time time for batch to return (ms)"
set xlabel "Threads spawned in batch"

set yrange [0:51]
set xrange [0:30]
plot \
    '1000-threads-20ms-spinloop.dat' with points pt '+' lc rgb '#aa000000' ps 1, \
    '1000-threads-20ms-sleep.dat' with points pt 6 lc rgb '#aa800000' ps 1

