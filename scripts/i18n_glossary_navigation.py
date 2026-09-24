"""Preserve existing navigation/quit translations when regenerating locales.

These strings already shipped in the generated YAML but were absent from the
source glossaries. Keep regeneration from replacing them with English.
"""

ENGLISH = [
    "Quit Orbit?",
    "Are you sure you want to quit Orbit?",
    "One or more agent sessions are still running. Quitting now will stop them. Completed session history is saved.",
    "Toggle Sidebar",
    "Forward",
    "Show the sessions sidebar. Toggle it from the top bar or with %{shortcut}.",
    "Focus Sessions Sidebar",
]
TRANSLATIONS = {
    "de": [
        "Orbit beenden?", "Möchtest du Orbit wirklich beenden?",
        "Mindestens eine Agentensitzung läuft noch. Wenn du Orbit jetzt beendest, wird sie angehalten. Der abgeschlossene Sitzungsverlauf bleibt gespeichert.",
        "Seitenleiste ein-/ausblenden", "Vorwärts",
        "Zeigt die Sitzungs-Seitenleiste. Umschalten über die obere Leiste oder mit %{shortcut}.",
        "Sitzungs-Seitenleiste fokussieren",
    ],
    "es": [
        "¿Salir de Orbit?", "¿Seguro que quieres salir de Orbit?",
        "Una o más sesiones del agente siguen en curso. Si sales ahora, se detendrán. El historial completado de las sesiones está guardado.",
        "Mostrar u ocultar la barra lateral", "Adelante",
        "Muestra la barra lateral de sesiones. Alterna desde la barra superior o con %{shortcut}.",
        "Enfocar la barra lateral de sesiones",
    ],
    "fr": [
        "Quitter Orbit ?", "Voulez-vous vraiment quitter Orbit ?",
        "Une ou plusieurs sessions de l’agent sont toujours en cours. Quitter maintenant les arrêtera. L’historique terminé des sessions est enregistré.",
        "Afficher/masquer la barre latérale", "Suivant",
        "Affiche la barre latérale des sessions. Basculez depuis la barre supérieure ou avec %{shortcut}.",
        "Focus sur la barre latérale des sessions",
    ],
    "it": [
        "Uscire da Orbit?", "Vuoi davvero uscire da Orbit?",
        "Una o più sessioni dell’agente sono ancora in corso. Uscendo ora verranno interrotte. La cronologia completata delle sessioni è salvata.",
        "Mostra/nascondi barra laterale", "Avanti",
        "Mostra la barra laterale delle sessioni. Attiva dalla barra superiore o con %{shortcut}.",
        "Vai alla barra laterale delle sessioni",
    ],
    "ja": [
        "Orbit を終了しますか？", "Orbit を終了してもよろしいですか？",
        "実行中のエージェントセッションがあります。今終了すると、それらは停止します。完了済みのセッション履歴は保存されています。",
        "サイドバーの表示を切り替え", "進む",
        "セッションのサイドバーを表示します。上部バーまたは %{shortcut} で切り替えられます。",
        "セッションのサイドバーにフォーカス",
    ],
    "ko": [
        "Orbit을 종료할까요?", "Orbit을 종료하시겠습니까?",
        "하나 이상의 에이전트 세션이 아직 실행 중입니다. 지금 종료하면 해당 세션이 중지됩니다. 완료된 세션 기록은 저장되어 있습니다.",
        "사이드바 표시/숨기기", "앞으로",
        "세션 사이드바를 표시합니다. 상단 바 또는 %{shortcut}(으)로 전환할 수 있습니다.",
        "세션 사이드바 포커스",
    ],
    "pt-BR": [
        "Encerrar o Orbit?", "Tem certeza de que deseja encerrar o Orbit?",
        "Uma ou mais sessões do agente ainda estão em andamento. Encerrar agora irá interrompê-las. O histórico concluído das sessões está salvo.",
        "Mostrar/ocultar barra lateral", "Avançar",
        "Mostra a barra lateral de sessões. Alterne pela barra superior ou com %{shortcut}.",
        "Focar a barra lateral de sessões",
    ],
    "ru": [
        "Выйти из Orbit?", "Вы уверены, что хотите выйти из Orbit?",
        "Один или несколько сеансов агента всё ещё выполняются. При выходе они будут остановлены. История завершённых сеансов сохранена.",
        "Показать/скрыть боковую панель", "Вперёд",
        "Показывает боковую панель сеансов. Переключайте с верхней панели или с помощью %{shortcut}.",
        "Перейти к боковой панели сеансов",
    ],
    "zh-CN": [
        "退出 Orbit？", "确定要退出 Orbit 吗？",
        "一个或多个代理会话仍在运行。现在退出将停止这些会话。已完成的会话历史记录会保留。",
        "显示或隐藏边栏", "前进",
        "显示会话侧边栏。可从顶栏或按 %{shortcut} 切换。",
        "聚焦会话侧边栏",
    ],
}
GLOSSARY = {locale: dict(zip(ENGLISH, translations)) for locale, translations in TRANSLATIONS.items()}
