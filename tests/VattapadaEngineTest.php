<?php
/**
 * VattapadaEngineTest — test suite for VattapadaEngine.
 *
 * Run: php tests/VattapadaEngineTest.php
 */

require_once __DIR__ . '/../legacy-php/CetasikaVector.php';
require_once __DIR__ . '/../legacy-php/VattapadaEngine.php';

// ---- Minimal test framework ----
$passed = 0;
$failed = 0;

function assertTrue(bool $condition, string $message): void {
    global $passed, $failed;
    if ($condition) {
        echo "  ✓ PASS: $message\n";
        $passed++;
    } else {
        echo "  ✗ FAIL: $message\n";
        $failed++;
    }
}

function assertBetween(float $value, float $min, float $max, string $message): void {
    assertTrue(
        $value >= $min && $value <= $max,
        "$message (got $value, expected [$min, $max])"
    );
}

// ---- Test 1: Wholesome vector ----
echo "\n[Test 1] Wholesome vector → high Indra Score\n";

$wholesome = new CetasikaVector();
$wholesome->activate(27, 0.9); // saddha
$wholesome->activate(28, 0.9); // sati
$wholesome->activate(29, 0.9); // hiri
$wholesome->activate(30, 0.9); // ottappa
$wholesome->activate(31, 0.9); // alobha
$wholesome->activate(32, 0.9); // adosa
$wholesome->activate(42, 0.9); // sammavaca
$wholesome->activate(47, 0.9); // panna

$engine = new VattapadaEngine(
    __DIR__ . '/../src/Vattapada-7.json',
    $wholesome
);

$score = $engine->indraScore();
assertTrue($score > 0.5, "Wholesome vector → Indra Score > 0.5");
assertTrue($score <= 1.0, "Wholesome vector → Indra Score <= 1.0");

// ---- Test 2: Unwholesome vector ----
echo "\n[Test 2] Unwholesome vector → low Indra Score\n";

$unwholesome = new CetasikaVector();
$unwholesome->activate(13, 0.9); // moha
$unwholesome->activate(17, 0.9); // lobha
$unwholesome->activate(20, 0.9); // dosa
$unwholesome->activate(22, 0.9); // macchariya

$engine2 = new VattapadaEngine(
    __DIR__ . '/../src/Vattapada-7.json',
    $unwholesome
);

$score2 = $engine2->indraScore();
assertTrue($score2 < 0.5, "Unwholesome vector → Indra Score < 0.5");
assertTrue($score2 >= 0.0, "Unwholesome vector → Indra Score >= 0.0");

// ---- Test 3: Scores are within [0.0, 1.0] ----
echo "\n[Test 3] All Vattapada scores are in [0.0, 1.0]\n";

$scores = $engine->evaluateAll();
foreach ($scores as $id => $s) {
    assertBetween($s, 0.0, 1.0, "$id score in range");
}

// ---- Test 4: 7 Vattapada practices are evaluated ----
echo "\n[Test 4] Exactly 7 Vattapada practices are evaluated\n";

assertTrue(count($scores) === 7, "There are exactly 7 Vattapada scores");

// ---- Test 5: Harmonic mean property ----
echo "\n[Test 5] Harmonic mean ≤ arithmetic mean\n";

$values = array_values($scores);
$arithmetic = array_sum($values) / count($values);
$harmonic = $engine->indraScore();

assertTrue(
    $harmonic <= $arithmetic + 1e-9,
    "Harmonic mean ($harmonic) ≤ arithmetic mean ($arithmetic)"
);

// ---- Summary ----
echo "\n========================================\n";
echo "  Passed: $passed\n";
echo "  Failed: $failed\n";
echo "========================================\n";

if ($failed > 0) {
    exit(1);
}

echo "\nAll tests passed.\n";
exit(0);
