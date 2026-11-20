package main

import (
	"encoding/json"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"time"
	"unicode"
)

type SnapshotItem struct {
	Codigo string `json:"codigo"`
	Fecha  string `json:"fecha"`
}

type MemoriaSpark struct {
	Notas       []string       `json:"notas"`
	Snapshots   []SnapshotItem `json:"snapshots"`
	Actualizado string         `json:"actualizado"`
}

var palabrasClaveFlame = []string{
	"raum", "modul", "schablone", "struktur", "aufbau", "abbau",
	"vorab", "fest", "mutable", "rolle", "gib", "leihe", "leihe_mut",
	"publico", "ergebnis", "gut", "fehler", "vielleicht", "algo", "nichts",
	"revisa", "kiste", "roh", "zeiger", "haupt", "eingabe!", "liste",
	"laos", "aus", "bis", "weiter", "abbruch", "ja", "aber",
	"und", "oder", "nicht", "wahr", "falsch", "versuch", "fang",
	"manifest!", "drucken!", "manifestation!", "manifiesto",
	"activision", "actionscript", "actende", "area", "Ripetox",
	"davay", "link", "linkcheck", "linkdrucken", "linkdriver",
	"driver", "aniverzal", "manus", "kaszchenein", "Ray", "conn",
	"mizushis", "Destroyer", "Dalf", "say", "Yagert", "Virecari", "Hakgest",
}

var accionesDash = []string{"activision", "actionscript", "actende"}

func rutaMemoria() string {
	home, err := os.UserHomeDir()
	if err != nil {
		return "spark_memoria.json"
	}
	return filepath.Join(home, ".spark_ia_memoria.json")
}

func cargarMemoria() MemoriaSpark {
	mem := MemoriaSpark{Notas: []string{}, Snapshots: []SnapshotItem{}}
	data, err := os.ReadFile(rutaMemoria())
	if err != nil {
		return mem
	}
	_ = json.Unmarshal(data, &mem)
	return mem
}

func guardarMemoria(mem MemoriaSpark) error {
	mem.Actualizado = time.Now().Format("2006-01-02 15:04:05")
	data, err := json.MarshalIndent(mem, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(rutaMemoria(), data, 0644)
}

func agregarSnapshot(mem *MemoriaSpark, codigo string) int {
	mem.Snapshots = append(mem.Snapshots, SnapshotItem{
		Codigo: codigo,
		Fecha:  time.Now().Format("2006-01-02 15:04:05"),
	})
	_ = guardarMemoria(*mem)
	return len(mem.Snapshots)
}

func distanciaLevenshtein(a, b string) int {
	ra, rb := []rune(strings.ToLower(a)), []rune(strings.ToLower(b))
	la, lb := len(ra), len(rb)
	dp := make([][]int, la+1)
	for i := range dp {
		dp[i] = make([]int, lb+1)
		dp[i][0] = i
	}
	for j := 0; j <= lb; j++ {
		dp[0][j] = j
	}
	for i := 1; i <= la; i++ {
		for j := 1; j <= lb; j++ {
			costo := 1
			if ra[i-1] == rb[j-1] {
				costo = 0
			}
			minVal := dp[i-1][j] + 1
			if dp[i][j-1]+1 < minVal {
				minVal = dp[i][j-1] + 1
			}
			if dp[i-1][j-1]+costo < minVal {
				minVal = dp[i-1][j-1] + costo
			}
			dp[i][j] = minVal
		}
	}
	return dp[la][lb]
}

func esZeitValido(val string) bool {
	v := strings.ToLower(strings.TrimSpace(val))
	if v == "inmediato" || v == "original" || v == "infinito" {
		return true
	}
	return regexp.MustCompile(`^\d+(?:\.\d+)?\.(ms|s|m|h)$`).MatchString(v)
}

func validarZeit(linea string, numLinea int) []string {
	var avisos []string
	reZeit := regexp.MustCompile(`~zeit\s*=\s*([^\s:\]]+)`)
	for _, m := range reZeit.FindAllStringSubmatch(linea, -1) {
		val := strings.TrimSpace(strings.TrimSuffix(m[1], "="))
		if val == "" {
			avisos = append(avisos, fmt.Sprintf("Línea %d: '~zeit=' está vacío.", numLinea))
		} else if !esZeitValido(val) {
			avisos = append(avisos, fmt.Sprintf("Línea %d: valor '%s' no válido en ~zeit.", numLinea, val))
		}
	}
	return avisos
}

func revisarAccionDash(linea string, numLinea int, reToken *regexp.Regexp) []string {
	base := linea
	if idx := strings.Index(base, "~zeit"); idx != -1 {
		base = base[:idx]
	} else if idx := strings.Index(base, "="); idx != -1 {
		base = base[:idx]
	}
	tokens := reToken.FindAllString(base, -1)
	if len(tokens) == 0 {
		return nil
	}
	cand := tokens[len(tokens)-1]
	for _, ok := range accionesDash {
		if strings.EqualFold(cand, ok) {
			return nil
		}
	}
	for _, ok := range accionesDash {
		if distanciaLevenshtein(cand, ok) <= 4 {
			return []string{fmt.Sprintf("Línea %d: '%s' parece mal escrito. ¿Querías decir '%s'?", numLinea, cand, ok)}
		}
	}
	return nil
}

func sugerirPalabraFlame(token string) string {
	limpio := strings.Trim(token, "(),[]{}:;=<>\"'~")
	if limpio == "" || strings.HasPrefix(limpio, "@") || strings.HasPrefix(limpio, "###") {
		return ""
	}
	for _, kw := range palabrasClaveFlame {
		if strings.EqualFold(limpio, kw) {
			return ""
		}
	}
	mejor, mejorDist := "", 999
	for _, kw := range palabrasClaveFlame {
		d := distanciaLevenshtein(limpio, kw)
		if d > 0 && d <= 2 && d < mejorDist {
			mejorDist, mejor = d, kw
		}
	}
	return mejor
}

func fmtNum(v float64) string {
	if math.Abs(v-math.Round(v)) < 1e-9 && math.Abs(v) < 1e15 {
		return fmt.Sprintf("%d", int64(math.Round(v)))
	}
	return strconv.FormatFloat(v, 'f', -1, 64)
}

type parser struct {
	s   string
	pos int
}

func (p *parser) sp() {
	for p.pos < len(p.s) && unicode.IsSpace(rune(p.s[p.pos])) {
		p.pos++
	}
}

func (p *parser) expr() (float64, error) {
	v, err := p.term()
	if err != nil {
		return 0, err
	}
	for {
		p.sp()
		if p.pos >= len(p.s) || (p.s[p.pos] != '+' && p.s[p.pos] != '-') {
			break
		}
		op := p.s[p.pos]
		p.pos++
		r, err := p.term()
		if err != nil {
			return 0, err
		}
		if op == '+' {
			v += r
		} else {
			v -= r
		}
	}
	return v, nil
}

func (p *parser) term() (float64, error) {
	v, err := p.pow()
	if err != nil {
		return 0, err
	}
	for {
		p.sp()
		if p.pos >= len(p.s) || (p.s[p.pos] != '*' && p.s[p.pos] != '/' && p.s[p.pos] != '%') {
			break
		}
		op := p.s[p.pos]
		p.pos++
		r, err := p.pow()
		if err != nil {
			return 0, err
		}
		switch op {
		case '*':
			v *= r
		case '/':
			if r == 0 {
				return 0, fmt.Errorf("No se puede dividir entre cero.")
			}
			v /= r
		case '%':
			if r == 0 {
				return 0, fmt.Errorf("No se puede calcular módulo entre cero.")
			}
			v = math.Mod(v, r)
		}
	}
	return v, nil
}

func (p *parser) pow() (float64, error) {
	p.sp()
	if p.pos < len(p.s) && (p.s[p.pos] == '+' || p.s[p.pos] == '-') {
		sign := 1.0
		if p.s[p.pos] == '-' {
			sign = -1.0
		}
		p.pos++
		v, err := p.pow()
		return sign * v, err
	}
	b, err := p.prim()
	if err != nil {
		return 0, err
	}
	p.sp()
	if p.pos < len(p.s) && p.s[p.pos] == '^' {
		p.pos++
		e, err := p.pow()
		if err != nil {
			return 0, err
		}
		return math.Pow(b, e), nil
	}
	return b, nil
}

func (p *parser) prim() (float64, error) {
	p.sp()
	if p.pos < len(p.s) && p.s[p.pos] == '(' {
		p.pos++
		v, err := p.expr()
		if err != nil {
			return 0, err
		}
		p.sp()
		if p.pos < len(p.s) && p.s[p.pos] == ')' {
			p.pos++
		}
		return v, nil
	}
	start := p.pos
	dot, dig := false, false
	for p.pos < len(p.s) {
		c := p.s[p.pos]
		if c >= '0' && c <= '9' {
			dig = true
			p.pos++
		} else if c == '.' && !dot {
			dot = true
			p.pos++
		} else {
			break
		}
	}
	if !dig {
		return 0, fmt.Errorf("inválido")
	}
	return strconv.ParseFloat(p.s[start:p.pos], 64)
}

func aMs(v float64, u string) (float64, bool) {
	switch strings.ToLower(u) {
	case "ms":
		return v, true
	case "s":
		return v * 1000, true
	case "m":
		return v * 60000, true
	case "h":
		return v * 3600000, true
	}
	return 0, false
}

func deMs(ms float64, u string) (string, bool) {
	switch strings.ToLower(u) {
	case "ms":
		return fmtNum(ms) + ".ms", true
	case "s":
		return fmtNum(ms/1000) + ".s", true
	case "m":
		return fmtNum(ms/60000) + ".m", true
	case "h":
		return fmtNum(ms/3600000) + ".h", true
	}
	return "", false
}

func resolverMate(texto string) (string, bool) {
	if m := regexp.MustCompile(`(?i)\b(?:raiz|sqrt)\s*\(?(-?\d+(?:\.\d+)?)\)?`).FindStringSubmatch(texto); len(m) == 2 {
		n, err := strconv.ParseFloat(m[1], 64)
		if err == nil {
			if n < 0 {
				return "No hay raíz real para números negativos.", true
			}
			return fmtNum(math.Sqrt(n)), true
		}
	}
	if m := regexp.MustCompile(`(?i)\bpromedio\s+([0-9.,\s-]+)`).FindStringSubmatch(texto); len(m) == 2 {
		partes := regexp.MustCompile(`-?\d+(?:\.\d+)?`).FindAllString(m[1], -1)
		if len(partes) > 0 {
			suma := 0.0
			for _, p := range partes {
				v, _ := strconv.ParseFloat(p, 64)
				suma += v
			}
			return fmtNum(suma / float64(len(partes))), true
		}
	}
	if m := regexp.MustCompile(`(?i)(\d+(?:\.\d+)?)\s*%\s*(?:de|del)\s*(\d+(?:\.\d+)?)`).FindStringSubmatch(texto); len(m) == 3 {
		p, _ := strconv.ParseFloat(m[1], 64)
		t, _ := strconv.ParseFloat(m[2], 64)
		return fmtNum((p / 100.0) * t), true
	}
	if m := regexp.MustCompile(`(?i)(\d+(?:\.\d+)?)\.(ms|s|m|h)[^\d\n]+(?:ripetox|repeticiones|veces)\s*=?\s*(\d+)`).FindStringSubmatch(texto); len(m) == 4 {
		d, _ := strconv.ParseFloat(m[1], 64)
		r, _ := strconv.ParseFloat(m[3], 64)
		return fmtNum(d*r) + "." + strings.ToLower(m[2]), true
	}
	if m := regexp.MustCompile(`(?i)(\d+(?:\.\d+)?)\.(ms|s|m|h)\s+(?:a|en|to)\s+\.?(ms|s|m|h)\b`).FindStringSubmatch(texto); len(m) == 4 {
		v, _ := strconv.ParseFloat(m[1], 64)
		if ms, ok := aMs(v, m[2]); ok {
			if out, ok2 := deMs(ms, m[3]); ok2 {
				return out, true
			}
		}
	}
	limpio := strings.ReplaceAll(strings.ReplaceAll(texto, "×", "*"), "**", "^")
	expr := regexp.MustCompile(`(?:-?\d+(?:\.\d+)?|\()[\d\s+\-*/%^().]*[\d\)]`).FindString(limpio)
	if expr == "" || !strings.ContainsAny(expr, "+-*/%^") {
		return "", false
	}
	res, err := (&parser{s: expr}).expr()
	if err != nil {
		if strings.Contains(err.Error(), "cero") {
			return err.Error(), true
		}
		return "", false
	}
	return fmtNum(res), true
}

func glosarioTema(tema string) (string, bool) {
t := strings.ToLower(strings.TrimSpace(tema))
switch t {
case "zeit", "~zeit":
return "~zeit: modificador de tiempo (inmediato, original, infinito o .ms, .s, .m, .h).", true
case "area":
return "area: define espacio o longitud, por ejemplo area= 512×400.", true
case "ripetox":
return "Ripetox: indica la cantidad de repeticiones, por ejemplo Ripetox=4.", true
case "activision":
return "activision: fase de activación dentro de flamedash.", true
case "actionscript":
return "actionscript: fase de ejecución de acción dentro de flamedash.", true
case "actende":
return "actende: fase de cierre o fin de acción dentro de flamedash.", true
case "haupt":
return "haupt: punto de entrada principal en flamecode.", true
default:
return "", false
}
}

func responder(texto string) string {
	if c, ok := resolverMate(texto); ok {
		return c
	}
	t := strings.ToLower(texto)
	for _, k := range []string{"zeit", "area", "ripetox", "activision", "actionscript", "actende", "haupt"} {
		if strings.Contains(t, k) && (strings.Contains(t, "qué") || strings.Contains(t, "que") || strings.Contains(t, "glosario") || strings.Contains(t, "sirve")) {
			if g, ok := glosarioTema(k); ok {
				return g
			}
		}
	}
	switch {
	case strings.Contains(t, "flamedash") || strings.Contains(t, "dash"):
		return "flamedash suma ~zeit (inmediato, original, infinito o .ms/.s/.m/.h), activision, actionscript, actende, area y Ripetox."
	case strings.Contains(t, "flamecode"):
		return "flamecode es el estilo base de FLAME y suele usar 'haupt' como entrada."
	case strings.Contains(t, "flameformel"):
		return "flameformel es el modo de fórmulas y reglas de FLAME."
	default:
		return "Puedo resolver cálculos (+, -, *, /, %, ^, raiz, promedio, %, ~zeit, Ripetox) o dudas básicas de FLAME."
	}
}

func limpiarLinea(l string) string {
	if idx := strings.Index(l, "###"); idx != -1 {
		l = l[:idx]
	}
	return regexp.MustCompile(`"[^"]*"|'[^']*'`).ReplaceAllString(l, `""`)
}

func analizarFlame(contenido string) []string {
	var avisos []string
	lineas := strings.Split(contenido, "\n")
	tieneHaupt, tieneStil, modo := false, false, ""
	reTok := regexp.MustCompile(`[A-Za-z_!]+`)
	comillas := 0

	for i, linea := range lineas {
		trim := strings.TrimSpace(linea)
		if trim == "" || strings.HasPrefix(trim, "###") || strings.HasPrefix(trim, "<") || strings.HasPrefix(trim, "</") || strings.HasPrefix(trim, "$=") {
			continue
		}
		comillas += strings.Count(trim, `"`)
		limpia := strings.TrimSpace(limpiarLinea(trim))
		if limpia == "" {
			continue
		}
		if strings.Contains(limpia, "haupt") {
			tieneHaupt = true
		}

		// Palabras repetidas seguidas en la misma línea
		toksLinea := reTok.FindAllString(limpia, -1)
		for idx := 1; idx < len(toksLinea); idx++ {
			if strings.EqualFold(toksLinea[idx], toksLinea[idx-1]) {
				avisos = append(avisos, fmt.Sprintf("Línea %d: palabra '%s' repetida seguida.", i+1, toksLinea[idx]))
			}
		}

		if strings.HasPrefix(strings.ToLower(limpia), "area") && strings.Contains(limpia, "=") {
			val := strings.TrimSpace(strings.SplitN(limpia, "=", 2)[1])
			if !regexp.MustCompile(`^\d+(?:\.\d+)?\s*[×xX]\s*\d+(?:\.\d+)?$`).MatchString(val) {
				avisos = append(avisos, fmt.Sprintf("Línea %d: formato de area '%s' poco usual (ej. 512×400).", i+1, val))
			}
			continue
		}
		if strings.HasPrefix(strings.ToLower(limpia), "ripetox") && strings.Contains(limpia, "=") {
			val := strings.TrimSpace(strings.SplitN(limpia, "=", 2)[1])
			if n, err := strconv.Atoi(val); err != nil || n < 1 {
				avisos = append(avisos, fmt.Sprintf("Línea %d: Ripetox '%s' debe ser entero positivo.", i+1, val))
			}
			continue
		}
		if strings.Contains(limpia, "~zeit") {
			avisos = append(avisos, revisarAccionDash(limpia, i+1, reTok)...)
			avisos = append(avisos, validarZeit(limpia, i+1)...)
			continue
		}
		if strings.HasPrefix(limpia, "!stil") {
			tieneStil = true
			partes := strings.Fields(limpia)
			if len(partes) < 2 {
				avisos = append(avisos, fmt.Sprintf("Línea %d: falta modo tras !stil.", i+1))
			} else {
				modo = partes[1]
				if modo != "flamecode" && modo != "flamedash" && modo != "flameformel" {
					avisos = append(avisos, fmt.Sprintf("Línea %d: modo '%s' no reconocido.", i+1, modo))
				}
			}
		}
		for _, tok := range toksLinea {
			if sug := sugerirPalabraFlame(tok); sug != "" {
				avisos = append(avisos, fmt.Sprintf("Línea %d: '%s' no coincide. ¿Querías decir '%s'?", i+1, tok, sug))
			}
		}
	}

	if comillas%2 != 0 {
		avisos = append(avisos, "Comillas: falta cerrar alguna comilla doble '\"'.")
	}
	if a, c := strings.Count(contenido, "["), strings.Count(contenido, "]"); a != c {
		avisos = append(avisos, fmt.Sprintf("Corchetes: %d '[' y %d ']'.", a, c))
	}
	if a, c := strings.Count(contenido, "("), strings.Count(contenido, ")"); a != c {
		avisos = append(avisos, fmt.Sprintf("Paréntesis: %d '(' y %d ')'.", a, c))
	}
	if a, c := strings.Count(contenido, "{"), strings.Count(contenido, "}"); a != c {
		avisos = append(avisos, fmt.Sprintf("Llaves: %d '{' y %d '}'.", a, c))
	}
	if !tieneStil {
		avisos = append(avisos, "Sugerencia: inicia con '!stil flamecode', '!stil flamedash' o '!stil flameformel'.")
	}
	if modo == "flamecode" && !tieneHaupt {
		avisos = append(avisos, "Aviso en flamecode: no se detectó 'haupt'.")
	}
	return avisos
}

func limpiarChat(contenido string) string {
	var out []string
	for _, l := range strings.Split(contenido, "\n") {
		t := strings.TrimSpace(l)
		if strings.HasPrefix(t, "!ayuda Spark") || strings.HasPrefix(t, "!Spark") ||
			strings.HasPrefix(t, "!hablando") || strings.HasPrefix(t, "!descartar Spark") ||
			strings.HasPrefix(t, "!borrando conversacion") || t == "!borrado" {
			continue
		}
		out = append(out, l)
	}
	return strings.Join(out, "\n")
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Spark.IA v0.3 - Comandos: analizar, ver, escanear, nuevo, glosario, snapshot, snapshots, recuperar [n], preguntar, notas, buscar-nota, borrar-nota <n>, estado, respaldar, acerca, comando")
		return
	}
	mem := cargarMemoria()
	switch os.Args[1] {
	case "acerca":
		fmt.Printf("Spark.IA v0.3 (Go / Termux) | Notas: %d | Snapshots: %d\n", len(mem.Notas), len(mem.Snapshots))

	case "estado":
		info, err := os.Stat(rutaMemoria())
		bytesMem := int64(0)
		if err == nil {
			bytesMem = info.Size()
		}
		fmt.Printf("Estado Spark.IA -> Notas: %d | Snapshots: %d | Memoria: %d bytes | Última act: %s\n",
			len(mem.Notas), len(mem.Snapshots), bytesMem, mem.Actualizado)

	case "respaldar":
		home, _ := os.UserHomeDir()
		dest := filepath.Join(home, "storage", "downloads")
		if err := os.MkdirAll(dest, 0755); err != nil {
			fmt.Println("No se pudo acceder a storage/downloads:", err)
			return
		}
		if data, err := os.ReadFile("main.go"); err == nil {
			_ = os.WriteFile(filepath.Join(dest, "spark_main.go"), data, 0644)
		}
		if bin, err := os.ReadFile("sparkia"); err == nil {
			_ = os.WriteFile(filepath.Join(dest, "sparkia"), bin, 0755)
		}
		fmt.Println("Spark.IA: Respaldo copiado a ~/storage/downloads (spark_main.go y sparkia).")

	case "ver":
		if len(os.Args) < 3 {
			fmt.Println("Indica archivo .fm")
			return
		}
		data, err := os.ReadFile(os.Args[2])
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		for i, l := range strings.Split(string(data), "\n") {
			fmt.Printf("%3d | %s\n", i+1, l)
		}

	case "escanear":
		entradas, err := os.ReadDir(".")
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		encontrados := 0
		for _, e := range entradas {
			if !e.IsDir() && strings.HasSuffix(e.Name(), ".fm") {
				encontrados++
				data, _ := os.ReadFile(e.Name())
				av := analizarFlame(string(data))
				fmt.Printf(" - %s (%d avisos)\n", e.Name(), len(av))
			}
		}
		if encontrados == 0 {
			fmt.Println("No hay archivos .fm en la carpeta actual.")
		}

	case "nuevo":
		if len(os.Args) < 3 {
			fmt.Println("Uso: sparkia nuevo <archivo.fm> [flamecode|flamedash|flameformel]")
			return
		}
		archivo := os.Args[2]
		modo := "flamedash"
		if len(os.Args) >= 4 {
			modo = os.Args[3]
		}
		var plant string
		switch modo {
		case "flamecode":
			plant = "!stil flamecode\nhaupt:\n  drucken! = \"Hola desde FLAME\":\n"
		case "flameformel":
			plant = "!stil flameformel\n### Fórmulas FLAME\n"
		default:
			plant = "!stil flamedash\narea= 512×400\nRipetox=1\nactivision~zeit=inmediato\nactionscript~zeit=1.s\nactende~zeit=original\n"
		}
		if err := os.WriteFile(archivo, []byte(plant), 0644); err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Printf("Spark.IA: Creado %s con plantilla '%s'.\n", archivo, modo)

	case "glosario":
		if len(os.Args) < 3 {
			fmt.Println("Temas: zeit, area, ripetox, activision, actionscript, actende, haupt")
			return
		}
		if desc, ok := glosarioTema(os.Args[2]); ok {
			fmt.Println("!Spark =", desc)
		} else {
			fmt.Println("!Spark = tema no encontrado en el glosario básico.")
		}

	case "analizar":
		if len(os.Args) < 3 {
			fmt.Println("Indica archivo .fm")
			return
		}
		data, err := os.ReadFile(os.Args[2])
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		avisos := analizarFlame(string(data))
		if len(avisos) == 0 {
			fmt.Println("Spark.IA: Código FLAME válido y sin errores detectados.")
			fmt.Printf("Siguiente paso sugerido: ./sparkia snapshot %s\n", os.Args[2])
		} else {
			fmt.Println("Spark.IA encontró lo siguiente:")
			for _, a := range avisos {
				fmt.Println(" -", a)
			}
			fmt.Printf("Siguiente paso sugerido: ./sparkia ver %s\n", os.Args[2])
		}

	case "snapshot":
		if len(os.Args) < 3 {
			fmt.Println("Indica archivo .fm")
			return
		}
		data, err := os.ReadFile(os.Args[2])
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		n := agregarSnapshot(&mem, string(data))
		fmt.Printf("Spark.IA: Snapshot #%d guardado (%s).\n", n, mem.Snapshots[n-1].Fecha)

	case "snapshots":
		if len(mem.Snapshots) == 0 {
			fmt.Println("Spark.IA: Sin snapshots.")
			return
		}
		for i, s := range mem.Snapshots {
			fmt.Printf("  #%d [%s] -> %s\n", i+1, s.Fecha, strings.Split(s.Codigo, "\n")[0])
		}

	case "recuperar":
		if len(mem.Snapshots) == 0 {
			fmt.Println("Spark.IA: Sin snapshots.")
			return
		}
		idx := len(mem.Snapshots) - 1
		if len(os.Args) >= 3 {
			n, err := strconv.Atoi(os.Args[2])
			if err != nil || n < 1 || n > len(mem.Snapshots) {
				fmt.Printf("Indica un número entre 1 y %d.\n", len(mem.Snapshots))
				return
			}
			idx = n - 1
		}
		fmt.Printf("Spark.IA: Snapshot #%d (%s):\n---\n%s\n", idx+1, mem.Snapshots[idx].Fecha, mem.Snapshots[idx].Codigo)

	case "notas":
		if len(mem.Notas) == 0 {
			fmt.Println("!Spark = sin notas.")
			return
		}
		for i, n := range mem.Notas {
			fmt.Printf("  %d. %s\n", i+1, n)
		}

	case "buscar-nota":
		if len(os.Args) < 3 {
			fmt.Println("Indica palabra a buscar en notas.")
			return
		}
		q := strings.ToLower(strings.Join(os.Args[2:], " "))
		halladas := 0
		for i, n := range mem.Notas {
			if strings.Contains(strings.ToLower(n), q) {
				fmt.Printf("  #%d: %s\n", i+1, n)
				halladas++
			}
		}
		if halladas == 0 {
			fmt.Println("!Spark = no encontré notas con ese texto.")
		}

	case "borrar-nota":
		if len(os.Args) < 3 {
			fmt.Println("Indica número de nota.")
			return
		}
		n, err := strconv.Atoi(os.Args[2])
		if err != nil || n < 1 || n > len(mem.Notas) {
			fmt.Println("!Spark = número inválido.")
			return
		}
		borrada := mem.Notas[n-1]
		mem.Notas = append(mem.Notas[:n-1], mem.Notas[n:]...)
		_ = guardarMemoria(mem)
		fmt.Printf("!Spark = nota #%d borrada: %q\n", n, borrada)

	case "preguntar":
		if len(os.Args) < 3 {
			fmt.Println("Escribe pregunta o cálculo.")
			return
		}
		fmt.Println("!Spark =", responder(strings.Join(os.Args[2:], " ")))

	case "comando":
		if len(os.Args) < 4 {
			fmt.Println("Uso: sparkia comando <archivo.fm> '<comando>'")
			return
		}
		archivo := os.Args[2]
		cmd := strings.TrimSpace(os.Args[3])
		switch {
		case strings.HasPrefix(cmd, "!ayuda Spark"):
			fmt.Println("!Spark = listo para FLAME. ¿Revisamos un .fm, hacemos un cálculo o guardamos un snapshot?")
		case strings.HasPrefix(cmd, "!hablando"):
			msg := strings.TrimSpace(strings.TrimPrefix(strings.TrimPrefix(cmd, "!hablando"), "="))
			if c, ok := resolverMate(msg); ok {
				fmt.Println("!Spark =", c)
				return
			}
			mem.Notas = append(mem.Notas, msg)
			_ = guardarMemoria(mem)
			fmt.Printf("!Spark = guardado en memoria: %q\n", msg)
		case strings.HasPrefix(cmd, "!descartar Spark"):
			if data, err := os.ReadFile(archivo); err == nil {
				n := agregarSnapshot(&mem, string(data))
				fmt.Printf("!Spark = auto-snapshot #%d guardado.\n", n)
				_ = os.WriteFile(archivo, []byte(limpiarChat(string(data))), 0644)
			}
			fmt.Println("!borrando conversacion del código...\n!borrado")
		case strings.HasPrefix(cmd, "!purga Spark"):
			mem = MemoriaSpark{Notas: []string{}, Snapshots: []SnapshotItem{}}
			_ = guardarMemoria(mem)
			fmt.Println("!Spark = memoria purgada por completo.")
		default:
			fmt.Println("!Spark = comando no reconocido.")
		}
	}
}
