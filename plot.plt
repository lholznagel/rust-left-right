set term wxt size 1000,1000 persist
set xlabel "Run"
set ylabel "Duration (in micros)"
set grid xtics ytics mxtics mytics
plot "out" u 1:2 w l t "leftright", \
     "out" u 1:3 w l t "std::mutex", \
     "out" u 1:4 w l t "std::rwlock", \
     "out" u 1:5 w l t "tokio::mutex", \
     "out" u 1:6 w l t "tokio::rwlock"
