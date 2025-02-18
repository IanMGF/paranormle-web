import json
from datetime import timedelta, datetime

import requests
import bs4
from typing import TypedDict, Final

from bs4 import NavigableString, Tag, PageElement


class Episode(TypedDict):
    cover_url: str
    cover_path: str
    title: str
    campaign: str
    date: str
    number: int
    duration: int

episode_list: list[Episode] = []
BASE_URL: Final[str] = "https://ordemparanormal.fandom.com"

campaign_sub_urls: Final[tuple[str, ...]] = (
    "/wiki/Categoria:Epis%C3%B3dios_de_A_Ordem_Paranormal",
    "/wiki/Categoria:Epis%C3%B3dios_de_O_Segredo_na_Floresta",
    "/wiki/Categoria:Episódios_de_Ordem_Paranormal:_Desconjuração",
    "/wiki/Categoria:Epis%C3%B3dios_de_Ordem_Paranormal:_Calamidade",
    "/wiki/Categoria:Epis%C3%B3dios_de_O_Segredo_na_Ilha"
    "/wiki/Categoria:Epis%C3%B3dios_de_Ordem_Paranormal:_Quarentena",
    "/wiki/Categoria:Epis%C3%B3dios_de_Sinais_do_Outro_Lado"
)

episode_urls = []

for sub_url in campaign_sub_urls:
    req = requests.get(BASE_URL + sub_url)
    soup = bs4.BeautifulSoup(req.text, features="html.parser")

    parents = soup.find_all("ul", {"class": "category-page__members-for-char"})

    all_eps_in_season = []
    for parent in parents:
        episodes = parent.find_all("li", {"class": "category-page__member"})
        all_eps_in_season.extend(episodes)

    eps_in_season_urls = []
    for ep in all_eps_in_season:
        eps_in_season_urls.append(ep.find("a").attrs["href"])

    episode_urls.extend(eps_in_season_urls)

for sub_url in episode_urls:
    req = requests.get(BASE_URL + sub_url)
    soup = bs4.BeautifulSoup(req.text, features="html.parser")

    # Title
    title_nodes = soup.find_all(attrs={"data-source": "título"})
    if len(title_nodes) != 1:
        print(f"Error in Title: {sub_url}")
        continue

    title = title_nodes[0].text

    # Campaign
    campaign_divs = soup.find_all("div", {"data-source": "???"})
    if len(campaign_divs) != 1:
        print(f"Error in Campaign: {sub_url}")
        continue

    campaign = campaign_divs[0].find("div", {"class": "pi-data-value"}).find("a").text
    del campaign_divs

    # Ep. Number
    number_div = soup.find("div", {"data-source": "episódio"})
    number = number_div.find("div", {"class": "pi-data-value"}).text
    number = int(number)
    del number_div

    # Airing Date
    date_div = soup.find("div", {"data-source": "exibição"})
    date = date_div.find("div", {"class": "pi-data-value"}).text
    del date_div

    # Duration
    duration_div = soup.find("div", {"data-source": "duração"})
    duration_list = duration_div.find("div", {"class": "pi-data-value"}).find("ul").find_all()

    durations: "list[PageElement | Tag | NavigableString]" = list(filter(
        lambda node: "(Twitch)" in node.text,
        duration_list
    ))

    if len(durations) != 1:
        print(f"Error in Length: {sub_url}")
        continue

    duration = durations[0].text[:-9]
    duration = datetime.strptime(duration, "%H:%M:%S")
    duration = timedelta(hours=duration.hour, minutes=duration.minute, seconds=duration.second)
    duration = int(duration.total_seconds())

    del duration_div
    del duration_list
    del durations

    # URL de Imagem
    image_nodes = soup.find_all(attrs={"data-source": "imagem"})
    if len(image_nodes) != 1:
        print(f"Error in Image: {sub_url}")
        continue

    image_url = image_nodes[0].find("a", attrs={"class": "image image-thumbnail"}).attrs["href"]

    img = requests.get(image_url, allow_redirects=True)
    with open(f"covers/{campaign}{number}.png", "wb") as img_file:
        img_file.write(img.content)

    cover_path = f"covers/{campaign}{number}.png"

    episode = Episode(
        cover_url=image_url,
        cover_path=cover_path,
        title=title,
        campaign=campaign,
        date=date,
        number=number,
        duration=duration,
    )

    episode_list.append(episode)

with open("episodes.json", "w", encoding="utf8") as episodes_file:
    json.dump(episode_list, episodes_file, indent=4, ensure_ascii=False)
