argm5 c.m5.meta > c.m5.verbose || exit 1
sed -i 's/%;/\n%:/g' c.m5.verbose || exit 1
yes | cp -f c.m5.verbose c.m5 || exit 1
sed -i 's/(print)//g' c.m5 || exit 1