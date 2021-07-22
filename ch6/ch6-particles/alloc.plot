set terminal svg size 1920,1080 fname 'Inter,14'
set output 'ch6-alloc-durations.svg'

# GENERAL SETTINGS
set key off
set rmargin 5
set grid ytics noxtics nocbtics back
set border 3 back lw 2 lc rgbcolor "#222222"

# X AXIS
set xlabel "Allocation size (bytes)"
set logscale x 2
set xtics nomirror out
set xrange [1 to 100000]

# Y AXIS
set ylabel "Allocation duration (ns)"
set logscale y
set yrange [10 to 10000]
set ytics nomirror out


plot "alloc.tsv" with points \
    pointtype 6 \
    pointsize 1.25 \
    linecolor rgbcolor "#22dd3131"
print "done"
