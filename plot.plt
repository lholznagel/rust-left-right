set term wxt size 900,900 persist
set xlabel "Run"
set ylabel "Duration (micros)"
set grid xtics ytics mxtics mytics
plot "out" u 1:2 w l t "std::mutex", \
     "out" u 1:3 w l t "tokio::mutex", \
     "out" u 1:4 w l t "std::rwlock", \
     "out" u 1:5 w l t "tokio::rwlock", \
     "out" u 1:6 w l t "leftright"
