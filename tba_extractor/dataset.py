import os
import sys
from argparse import Namespace
from pprint import pprint
from typing import Dict, Callable, List

from tba_extractor.csv_data import CsvData
from tba_extractor.option import Option, Switch
from tba_extractor.tba import TheBlueApi

API_KEY = os.environ.get('TBA_API_KEY', '').strip()

if not API_KEY:
    print("Please set TBA_API_KEY environment variable.")
    sys.exit(1)

TBA = TheBlueApi(API_KEY)

datasets: Dict[str, 'DataSet'] = {
}


class DataSet:
    def __init__(self,
                 name: str,
                 options: List[Option],
                 func: Callable[[Namespace], CsvData]):
        self.name = name
        self.options = options
        self.func = func

        datasets[name] = self


def _dataset(name: str, options: List[Option]):
    def decorate(f: Callable[[Namespace], CsvData]):
        return DataSet(name, options, f)

    return decorate


def get_team_number(team_key: str) -> int:
    team = TBA.get(f'/team/{team_key}').json()
    return team['team_number']


@_dataset(
    name='basic_matches',
    options=[
        Option('--event-key', 'Event key.')
    ]
)
def basic_matches(args: Namespace) -> CsvData:
    event_key = args.event_key
    matches = TBA.get(f'/event/{event_key}/matches/simple').json()
    matches = filter(lambda m: m['comp_level'] == 'qm', matches)
    teams = {team['key']: team['team_number'] for team in TBA.get(f'/event/{event_key}/teams/simple').json()}
    result = [['Team #', 'Match #']]
    for match in sorted(matches, key=lambda m: m['match_number']):
        team_keys = match['alliances']['blue']['team_keys'] + match['alliances']['red']['team_keys']
        for team_number in map(teams.get, team_keys):
            result.append([team_number, match['match_number']])
    return CsvData.new(result)


@_dataset(
    name='2019_match_climb_data',
    options=[
        Option('--event-key', 'Event key.'),
        Switch('--just-climb', 'Extract just the climb data, no team/match info.')
    ]
)
def _2019_match_climb_data(args: Namespace) -> CsvData:
    event_key = args.event_key
    just_climb = getattr(args, 'just_climb', False)
    matches = TBA.get(f'/event/{event_key}/matches').json()
    matches = filter(lambda m: m['comp_level'] == 'qm', matches)
    teams = {team['key']: team['team_number'] for team in TBA.get(f'/event/{event_key}/teams/simple').json()}
    result = [['Climb Level']] if just_climb else [['Team #', 'Match #', 'Climb Level']]
    for match in sorted(matches, key=lambda m: m['match_number']):
        if match['score_breakdown'] is None:
            continue
        for a in ['blue', 'red']:
            rows = _extract_tmc_row(teams, match, a)
            if just_climb:
                rows = [r[2] for r in rows]
            result.extend(rows)
    return CsvData.new(result)


@_dataset(
    name='2019_match_fouls',
    options=[
        Option('--event-key', 'Event key.'),
        Switch('--just-fouls', 'Extract just the foul data, no team/match info.')
    ]
)
def _2019_match_fouls(args: Namespace) -> CsvData:
    event_key = args.event_key
    just_fouls = getattr(args, 'just_fouls', False)
    matches = TBA.get(f'/event/{event_key}/matches').json()
    matches = filter(lambda m: m['comp_level'] == 'qm', matches)
    teams = {team['key']: team['team_number'] for team in TBA.get(f'/event/{event_key}/teams/simple').json()}
    result = [['Fouls']] if just_fouls else [['Team #', 'Match #', 'Fouls']]
    for match in sorted(matches, key=lambda m: m['match_number']):
        if match['score_breakdown'] is None:
            continue
        for a in ['blue', 'red']:
            rows = _extract_tmf_row(teams, match, a)
            if just_fouls:
                rows = [r[2] for r in rows]
            result.extend(rows)
    return CsvData.new(result)


def _extract_tmc_row(teams, match, alliance):
    asb = match['score_breakdown'][alliance]
    return [
        data[:2] + ("0" if data[2] == 'None' else data[2].replace('HabLevel', ''),)
        for data in
        ((team_number, match['match_number'], asb[f'endgameRobot{i + 1}'])
         for i, team_number in enumerate(map(teams.get, match['alliances'][alliance]['team_keys'])))
    ]


def _extract_tmf_row(teams, match, alliance):
    asb = match['score_breakdown'][alliance]
    return [
        (team_number, match['match_number'], asb['foulCount'])
         for i, team_number in enumerate(map(teams.get, match['alliances'][alliance]['team_keys']))
    ]


__all__ = ['datasets']
