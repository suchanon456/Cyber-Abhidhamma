<?php
/**
 * VattapadaEngine — Dynamic Scoring Engine for the 7 Vattapada practices.
 *
 * Loads Vattapada-7.json policy spec and evaluates a CetasikaVector
 * against the 7 practices, computing an Indra Score.
 *
 * @package CyberAbhidhamma
 */

require_once __DIR__ . '/CetasikaVector.php';

class VattapadaEngine
{
    /** @var array The loaded policy spec. */
    private array $policy;

    /** @var CetasikaVector The current cetasika vector. */
    private CetasikaVector $vector;

    /**
     * @param string $policyPath Path to Vattapada-7.json
     * @param CetasikaVector $vector The vector to evaluate
     * @throws RuntimeException If the policy file cannot be loaded.
     */
    public function __construct(string $policyPath, CetasikaVector $vector)
    {
        if (!file_exists($policyPath)) {
            throw new RuntimeException("Policy file not found: {$policyPath}");
        }
        $this->policy = json_decode(file_get_contents($policyPath), true);
        $this->vector = $vector;
    }

    /**
     * Evaluate all 7 vattas and return scores.
     *
     * @return array<string, float> vatta_1..vatta_7 => score
     */
    public function evaluateAll(): array
    {
        $scores = [];
        foreach ($this->policy['vattapada_7'] as $vatta) {
            $id = $vatta['id'];
            $scores[$id] = $this->evaluateVatta($vatta);
        }
        return $scores;
    }

    /**
     * Evaluate a single vatta using its cetasika_formula.
     *
     * @param array $vatta The vatta definition from the policy.
     * @return float Score in [0.0, 1.0]
     */
    private function evaluateVatta(array $vatta): float
    {
        $formula = $vatta['cetasika_formula'];
        $weights = $formula['weights'];
        $penalties = $formula['penalties'];

        $score = 0.0;

        // Positive contributions
        foreach ($weights as $name => $weight) {
            $index = $this->indexOf($name);
            if ($index !== null) {
                $score += $this->vector->toArray()[$index] * $weight;
            }
        }

        // Negative contributions
        foreach ($penalties as $name => $weight) {
            $index = $this->indexOf($name);
            if ($index !== null) {
                $score += $this->vector->toArray()[$index] * $weight;
                // weight is already negative in the spec
            }
        }

        // Clamp to [0.0, 1.0]
        return max(0.0, min(1.0, $score));
    }

    /**
     * Compute the Indra Score using weighted harmonic mean.
     *
     * @return float
     */
    public function indraScore(): float
    {
        $scores = array_values($this->evaluateAll());
        $n = count($scores);
        if ($n === 0) {
            return 0.0;
        }
        $sum = 0.0;
        foreach ($scores as $s) {
            if ($s <= 0.0) {
                return 0.0; // harmonic mean is 0 if any value is 0
            }
            $sum += 1.0 / $s;
        }
        return $n / $sum;
    }

    /**
     * Map a cetasika name to its index in the canonical 52.
     *
     * @param string $name
     * @return int|null
     */
    private function indexOf(string $name): ?int
    {
        static $map = [
            'phassa' => 0, 'vedana' => 1, 'sanna' => 2, 'cetana' => 3,
            'ekaggata' => 4, 'jivitindriya' => 5, 'manasikara' => 6,
            'vitakka' => 7, 'vicara' => 8, 'adhimokkha' => 9,
            'viriya' => 10, 'piti' => 11, 'chanda' => 12,
            'moha' => 13, 'ahirika' => 14, 'anottappa' => 15, 'uddhacca' => 16,
            'lobha' => 17, 'ditthi' => 18, 'mana' => 19,
            'dosa' => 20, 'issa' => 21, 'macchariya' => 22, 'kukkucca' => 23,
            'thina' => 24, 'middha' => 25, 'vicikiccha' => 26,
            'saddha' => 27, 'sati' => 28, 'hiri' => 29, 'ottappa' => 30,
            'alobha' => 31, 'adosa' => 32, 'tatramajjhattata' => 33,
            'kayapassaddhi' => 34, 'cittapassaddhi' => 35,
            'kayakammannata' => 36, 'cittakammannata' => 37,
            'kayapagunnata' => 38, 'cittapagunnata' => 39,
            'kayujukata' => 40, 'cittujukata' => 41,
            'sammavaca' => 42, 'sammakammanta' => 43, 'samma_ajiva' => 44,
            'karuna' => 45, 'mudita' => 46, 'panna' => 47,
        ];
        return $map[$name] ?? null;
    }
}
