TESTFLAGS = --nocapture
UNDIZZY   = cargo run undizzy
DIZZY     = cargo run dizzy

test:
		cargo test -- ${TESTFLAGS}

testUndizzy:
		 ${UNDIZZY} -f src/test.xyz & ${UNDIZZY} -f src/split_sw.xyz

testDizzy:
		 ${DIZZY} "TFCCTF{th15_ch4ll3ng3_m4k3s_m3_d1zzy_;d}" & ${DIZZY} "rust test dizzy!"

