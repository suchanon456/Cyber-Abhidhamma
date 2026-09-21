<?php
/**
 * CetasikaVector — 52-dimensional behavioral state vector.
 *
 * Maps the 52 Abhidhamma cetasikas to a computable feature vector,
 * usable for intent-based analysis, behavioral classification,
 * and deterministic state machines.
 *
 * @package CyberAbhidhamma
 */

class CetasikaVector
{
    /** Total number of cetasikas in Abhidhamma. */
    public const DIMENSIONS = 52;

    /**
     * Index ranges for the three canonical groups.
     * Based on the Abhidhamma classification of 52 cetasikas.
     */
    public const GROUP_ANNASAMANA_START = 0;   // 13 cetasikas
    public const GROUP_ANNASAMANA_END   = 12;
    public const GROUP_AKUSALA_START    = 13;  // 14 cetasikas
    public const GROUP_AKUSALA_END      = 26;
    public const GROUP_SOBHANA_START    = 27;  // 25 cetasikas
    public const GROUP_SOBHANA_END      = 51;

    /**
     * Named indices for the 7 Sabbacittasadharana cetasikas
     * (universal mental factors present in every citta).
     */
    public const PHASSA     = 0;
    public const VEDANA     = 1;
    public const SANNA      = 2;
    public const CETANA     = 3;
    public const EKAGGATA   = 4;
    public const JIVITINDRIYA = 5;
    public const MANASIKARA = 6;

    /** 52-dimensional state vector. */
    private array $vector;

    /**
     * Initialize the vector with all cetasikas inactive (weight = 0.0).
     */
    public function __construct()
    {
        $this->vector = array_fill(0, self::DIMENSIONS, 0.0);
    }

    /**
     * Activate a cetasika at the given index with a weight.
     *
     * @param int   $index  Index in the range [0, 51].
     * @param float $weight Activation weight in the range [0.0, 1.0].
     * @throws InvalidArgumentException If index or weight is out of range.
     */
    public function activate(int $index, float $weight = 1.0): void
    {
        if ($index < 0 || $index >= self::DIMENSIONS) {
            throw new InvalidArgumentException(
                "Cetasika index must be between 0 and " . (self::DIMENSIONS - 1)
            );
        }
        if ($weight < 0.0 || $weight > 1.0) {
            throw new InvalidArgumentException(
                "Weight must be between 0.0 and 1.0"
            );
        }
        $this->vector[$index] = $weight;
    }

    /**
     * Deactivate a cetasika (set weight to 0.0).
     */
    public function deactivate(int $index): void
    {
        $this->activate($index, 0.0);
    }

    /**
     * Reset all cetasikas to inactive.
     */
    public function reset(): void
    {
        $this->vector = array_fill(0, self::DIMENSIONS, 0.0);
    }

    /**
     * Return the full 52-dimensional vector.
     *
     * @return float[]
     */
    public function toArray(): array
    {
        return $this->vector;
    }

    /**
     * Return the total weight of cetasikas in the akusala group (14).
     */
    public function akusalaWeight(): float
    {
        return array_sum(array_slice(
            $this->vector,
            self::GROUP_AKUSALA_START,
            self::GROUP_AKUSALA_END - self::GROUP_AKUSALA_START + 1
        ));
    }

    /**
     * Return the total weight of cetasikas in the sobhana group (25).
     */
    public function sobhanaWeight(): float
    {
        return array_sum(array_slice(
            $this->vector,
            self::GROUP_SOBHANA_START,
            self::GROUP_SOBHANA_END - self::GROUP_SOBHANA_START + 1
        ));
    }

    /**
     * Determine whether the current state is akusala-dominant.
     */
    public function isAkusalaDominant(): bool
    {
        return $this->akusalaWeight() > $this->sobhanaWeight();
    }

    /**
     * Determine whether the current state is sobhana-dominant.
     */
    public function isSobhanaDominant(): bool
    {
        return $this->sobhanaWeight() > $this->akusalaWeight();
    }

    /**
     * Return a human-readable summary of the current state.
     */
    public function summarize(): string
    {
        return sprintf(
            "CetasikaVector [akusala=%.2f, sobhana=%.2f, dominant=%s]",
            $this->akusalaWeight(),
            $this->sobhanaWeight(),
            $this->isAkusalaDominant() ? "akusala" : ($this->isSobhanaDominant() ? "sobhana" : "neutral")
        );
    }
}
