#!/usr/bin/env python3
"""Illustrative monetary dynamics for the Essentia research program.

This is not a forecast and does not encode a preferred policy result. It tests a
small set of identities and hypotheses that Part B says must be investigated:

1. Nominal denomination is not real value.
2. Indexed 𝒰 -> ℛ -> ℰ settlement can be scale-invariant.
3. Inflation pressure depends on activated real claims relative to deliverable
   output, desired balances, velocity, and confidence, not on a large nominal
   token count by itself.
4. Supply response can change how much new demand appears as output or prices.
5. Monetary adoption can have both low-acceptance and high-acceptance equilibria.

The equations are deliberately visible and incomplete. They are a starting point
for falsification, sensitivity analysis, and replacement by better models.
"""

from __future__ import annotations

import argparse
import csv
import json
import math
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable


@dataclass(frozen=True)
class MonetaryParameters:
    periods: int = 36
    population: int = 1_000
    entitlement_u_per_person: float = 1.0
    activation_rate: float = 1.0
    initial_e_per_u: float = 1.0
    initial_capacity_u: float = 1_000.0
    price_adjustment: float = 0.25
    capacity_response: float = 0.0
    producer_credit_response: float = 0.0
    depreciation_rate: float = 0.0
    existing_balance_demand_u: float = 0.0


@dataclass(frozen=True)
class AdoptionParameters:
    periods: int = 48
    initial_acceptance: float = 0.05
    intrinsic_utility: float = -2.0
    network_effect: float = 5.0
    provider_coverage: float = 0.25
    provider_effect: float = 3.0
    friction: float = 0.8
    instability: float = 0.0
    adjustment_speed: float = 0.35


def _validate_fraction(name: str, value: float) -> None:
    if not 0.0 <= value <= 1.0:
        raise ValueError(f"{name} must be between 0 and 1")


def simulate_monetary_path(params: MonetaryParameters) -> list[dict[str, float]]:
    """Run a minimal indexed-settlement and real-capacity model.

    Activated 𝒰 is a real claim. ℰ settlement equals that claim multiplied by
    the current nominal ℰ/𝒰 rate. Prices respond only to unmet real demand in
    this minimal version. That assumption is intentionally restrictive: later
    models should add desired money balances, heterogeneous spending, inventories,
    imports, expectations, market power, and explicit provider balance sheets.
    """

    if params.periods <= 0 or params.population <= 0:
        raise ValueError("periods and population must be positive")
    if params.entitlement_u_per_person < 0 or params.initial_capacity_u < 0:
        raise ValueError("entitlement and capacity must be non-negative")
    if params.initial_e_per_u <= 0:
        raise ValueError("initial_e_per_u must be positive")
    _validate_fraction("activation_rate", params.activation_rate)
    _validate_fraction("depreciation_rate", params.depreciation_rate)

    e_per_u = params.initial_e_per_u
    capacity_u = params.initial_capacity_u
    activated_u = (
        params.population
        * params.entitlement_u_per_person
        * params.activation_rate
    )
    rows: list[dict[str, float]] = []

    for period in range(params.periods):
        real_demand_u = activated_u + params.existing_balance_demand_u
        delivered_u = min(capacity_u, real_demand_u)
        shortage_u = max(0.0, real_demand_u - delivered_u)

        if delivered_u > 0:
            shortage_ratio = shortage_u / delivered_u
        elif real_demand_u > 0:
            shortage_ratio = 1.0
        else:
            shortage_ratio = 0.0

        settlement_rate_e_per_u = e_per_u
        civic_issue_e = activated_u * settlement_rate_e_per_u
        closing_rate_e_per_u = settlement_rate_e_per_u * math.exp(
            params.price_adjustment * shortage_ratio
        )

        rows.append(
            {
                "period": float(period),
                "activated_u": activated_u,
                "real_demand_u": real_demand_u,
                "capacity_u": capacity_u,
                "delivered_u": delivered_u,
                "shortage_u": shortage_u,
                "shortage_ratio": shortage_ratio,
                "settlement_rate_e_per_u": settlement_rate_e_per_u,
                "closing_rate_e_per_u": closing_rate_e_per_u,
                "normalized_e_per_u": (
                    closing_rate_e_per_u / params.initial_e_per_u
                ),
                "civic_issue_e": civic_issue_e,
                "real_value_of_issue_u": (
                    civic_issue_e / settlement_rate_e_per_u
                    if settlement_rate_e_per_u
                    else 0.0
                ),
            }
        )

        e_per_u = closing_rate_e_per_u

        capacity_gap_u = max(0.0, real_demand_u - capacity_u)
        expansion_u = (
            params.capacity_response + params.producer_credit_response
        ) * capacity_gap_u
        capacity_u = max(
            0.0,
            capacity_u * (1.0 - params.depreciation_rate) + expansion_u,
        )

    return rows


def simulate_adoption(params: AdoptionParameters) -> list[dict[str, float]]:
    """Run a transparent coordination-threshold model.

    Acceptance rises when direct utility, provider coverage, and expected network
    acceptance outweigh friction and instability. The logistic form allows both
    low- and high-acceptance paths. Parameters are hypotheses, not estimates.
    """

    if params.periods <= 0:
        raise ValueError("periods must be positive")
    _validate_fraction("initial_acceptance", params.initial_acceptance)
    _validate_fraction("provider_coverage", params.provider_coverage)
    _validate_fraction("adjustment_speed", params.adjustment_speed)

    acceptance = params.initial_acceptance
    rows: list[dict[str, float]] = []

    for period in range(params.periods):
        latent_utility = (
            params.intrinsic_utility
            + params.network_effect * acceptance
            + params.provider_effect * params.provider_coverage
            - params.friction
            - params.instability
        )
        target_acceptance = 1.0 / (1.0 + math.exp(-latent_utility))
        acceptance += params.adjustment_speed * (target_acceptance - acceptance)
        rows.append(
            {
                "period": float(period),
                "acceptance": acceptance,
                "target_acceptance": target_acceptance,
                "latent_utility": latent_utility,
            }
        )

    return rows


def default_scenarios() -> dict[str, MonetaryParameters]:
    return {
        "balanced_scale_1": MonetaryParameters(initial_e_per_u=1.0),
        "balanced_scale_1m": MonetaryParameters(initial_e_per_u=1_000_000.0),
        "fixed_shortage": MonetaryParameters(initial_capacity_u=700.0),
        "supply_response": MonetaryParameters(
            initial_capacity_u=700.0,
            capacity_response=0.35,
        ),
        "supply_plus_producer_credit": MonetaryParameters(
            initial_capacity_u=700.0,
            capacity_response=0.35,
            producer_credit_response=0.25,
        ),
        "real_overclaim": MonetaryParameters(
            entitlement_u_per_person=1.5,
            initial_capacity_u=1_000.0,
            capacity_response=0.15,
        ),
    }


def default_adoption_scenarios() -> dict[str, AdoptionParameters]:
    return {
        "isolated_wallets": AdoptionParameters(provider_coverage=0.10),
        "founding_market": AdoptionParameters(provider_coverage=0.55),
        "founding_market_high_friction": AdoptionParameters(
            provider_coverage=0.55,
            friction=2.0,
        ),
        "founding_market_instability": AdoptionParameters(
            provider_coverage=0.55,
            instability=1.25,
        ),
    }


def summarize_monetary(rows: Iterable[dict[str, float]]) -> dict[str, float]:
    materialized = list(rows)
    if not materialized:
        return {}
    final = materialized[-1]
    return {
        "ending_normalized_e_per_u": final["normalized_e_per_u"],
        "ending_capacity_u": final["capacity_u"],
        "cumulative_shortage_u": sum(row["shortage_u"] for row in materialized),
    }


def summarize_adoption(rows: Iterable[dict[str, float]]) -> dict[str, float]:
    materialized = list(rows)
    if not materialized:
        return {}
    return {"ending_acceptance": materialized[-1]["acceptance"]}


def run_self_test() -> None:
    scenarios = default_scenarios()
    scale_1 = simulate_monetary_path(scenarios["balanced_scale_1"])
    scale_1m = simulate_monetary_path(scenarios["balanced_scale_1m"])

    normalized_1 = [row["normalized_e_per_u"] for row in scale_1]
    normalized_1m = [row["normalized_e_per_u"] for row in scale_1m]
    assert normalized_1 == normalized_1m, "neutral denomination scale changed the real path"
    assert all(abs(value - 1.0) < 1e-12 for value in normalized_1)

    fixed = summarize_monetary(simulate_monetary_path(scenarios["fixed_shortage"]))
    response = summarize_monetary(simulate_monetary_path(scenarios["supply_response"]))
    credit = summarize_monetary(
        simulate_monetary_path(scenarios["supply_plus_producer_credit"])
    )
    assert response["cumulative_shortage_u"] < fixed["cumulative_shortage_u"]
    assert credit["cumulative_shortage_u"] < response["cumulative_shortage_u"]

    adoption = default_adoption_scenarios()
    isolated = summarize_adoption(simulate_adoption(adoption["isolated_wallets"]))
    market = summarize_adoption(simulate_adoption(adoption["founding_market"]))
    assert market["ending_acceptance"] > isolated["ending_acceptance"]


def write_csv(path: Path, rows: list[dict[str, float]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0].keys()))
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output-dir", type=Path)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        run_self_test()

    result: dict[str, object] = {"monetary": {}, "adoption": {}}

    for name, params in default_scenarios().items():
        rows = simulate_monetary_path(params)
        result["monetary"][name] = {
            "parameters": asdict(params),
            "summary": summarize_monetary(rows),
        }
        if args.output_dir:
            write_csv(args.output_dir / f"monetary-{name}.csv", rows)

    for name, params in default_adoption_scenarios().items():
        rows = simulate_adoption(params)
        result["adoption"][name] = {
            "parameters": asdict(params),
            "summary": summarize_adoption(rows),
        }
        if args.output_dir:
            write_csv(args.output_dir / f"adoption-{name}.csv", rows)

    if args.json:
        print(json.dumps(result, indent=2, sort_keys=True))
    else:
        for family, scenarios in result.items():
            print(f"[{family}]")
            for name, data in scenarios.items():
                print(name, json.dumps(data["summary"], sort_keys=True))


if __name__ == "__main__":
    main()
