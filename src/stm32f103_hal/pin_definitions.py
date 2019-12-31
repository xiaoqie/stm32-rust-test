FIRST = 'A'
LAST = 'G'

if __name__ == '__main__':
    for X in [chr(c) for c in range(ord(FIRST), ord(LAST) + 1)]:
        for x in range(16):
            crl_or_crh = "crl" if x < 8 else "crh"
            print(f"gpio_pin!(P{X}{x}, {crl_or_crh}, GPIO{X}, iop{X.lower()}en, mode{x}, cnf{x}, br{x}, bs{x}, idr{x}, odr{x});")
