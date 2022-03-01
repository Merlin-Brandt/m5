argswirl c.swirl.meta > c.swirl.verbose || exit 1
sed -i 's/%;/\n%:/g' c.swirl.verbose || exit 1
yes | cp -f c.swirl.verbose c.swirl || exit 1
sed -i 's/(print)//g' c.swirl || exit 1