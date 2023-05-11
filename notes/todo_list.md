- Comando para ligar/desligar a app_configurations.deploy_ready (similar ao /debug)
- Github Action deploy

---------------------Jukebox----------------------------------
JukeboxUse{id: Guid, guild_id: u64, user_id: u64, date: Date, youtube_title: String }
Implementar comandos:
-play, -p: adiciona ao fim da fila

-play-next, -pn: adiciona na fila após a musica atual

-queue, -q: mostra a fila

-stop, -s, -clear: para e limpa a fila

--------------------R34-----------------------------------
R34Use{id: Guid, guild_id: u64, user_id: u64, date: Date, post_title: String }
Implementar comandos:
-random, -r: pega imagem aleatoria
-search, -s: procura o primeiro post com o prompt passado
-spam: traz 15 posts com o prompt passado ou aleatorios se nao houver prompt

-------------------ML------------------------------
-treinar a i.a. pra reconhecer a voz e modo de falar da rapaziada
-comando receber prompt texto e mencionar quem foi que disse