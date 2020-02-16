


# GENERAL
set key off
set rmargin 5
set grid ytics noxtics nocbtics back
set border 3 back lw 2 lc rgbcolor "#222222"

# X AXIS
set xlabel "Allocation size (bytes)"
set logscale x 2
set xtics nomirror out

# Y AXIS
set ylabel "Time taken for malloc() to return"
set logscale y
set yrange [0.00005 to 0.0005]
set ytics ( \
    "50 {/Symbol m}s"   0.00005, \
    "75 {/Symbol m}s"   0.000075, \
    "100 {/Symbol m}s"  0.0001, \
    "250 {/Symbol m}s"  0.00025, \
    "500 {/Symbol m}s"  0.0005, \
)
    #"750 {/Symbol m}s"  0.00075, \
    "1000 {/Symbol m}s" 0.001, \
    #"2500 {/Symbol m}s" 0.0025, \
    "5000 {/Symbol m}s" 0.005  \
# )
set ytics nomirror out

plot "allocs.tsv" with points pointtype 6 linecolor rgbcolor "#aafa2a2a"
print "done"