"""Subagent session families and disk-only history."""
ENGLISH = [
    "Subagents (%{count})", "Subagents without a parent (%{count})", "Subagent",
    "Include subagents", "Hide subagents", "Refresh",
    "Read-only saved transcript. Refresh to see new messages; this view does not resume the agent.",
    "Loading saved transcript…",
    "Limited preview: first 8 MiB of the file, up to 12,000 characters per message.",
    "Copy preview", "No saved messages yet.",
]
TRANSLATIONS = {
    "zh-CN": [
        "子代理（%{count}）", "无父会话的子代理（%{count}）", "子代理", "包含子代理", "隐藏子代理", "刷新",
        "已保存对话的只读视图。刷新可查看新消息；此视图不会恢复代理运行。", "正在加载已保存的对话…",
        "预览受限：文件的前 8 MiB，每条消息最多 12,000 个字符。", "复制预览", "尚无已保存的消息。",
    ],
    "ja": [
        "サブエージェント（%{count}）", "親のないサブエージェント（%{count}）", "サブエージェント", "サブエージェントを含める", "サブエージェントを非表示", "更新",
        "保存された会話の読み取り専用表示です。更新すると新しいメッセージが表示されます。エージェントの実行は再開しません。", "保存された会話を読み込み中…",
        "プレビュー制限：ファイルの先頭 8 MiB、各メッセージ最大 12,000 文字。", "プレビューをコピー", "保存されたメッセージはまだありません。",
    ],
    "ko": [
        "하위 에이전트 (%{count})", "상위 세션이 없는 하위 에이전트 (%{count})", "하위 에이전트", "하위 에이전트 포함", "하위 에이전트 숨기기", "새로 고침",
        "저장된 대화의 읽기 전용 보기입니다. 새 메시지를 보려면 새로 고침하세요. 에이전트 실행은 재개하지 않습니다.", "저장된 대화 불러오는 중…",
        "제한된 미리보기: 파일의 처음 8 MiB, 메시지당 최대 12,000자.", "미리보기 복사", "아직 저장된 메시지가 없습니다.",
    ],
    "es": [
        "Subagentes (%{count})", "Subagentes sin sesión principal (%{count})", "Subagente", "Incluir subagentes", "Ocultar subagentes", "Actualizar",
        "Conversación guardada de solo lectura. Actualiza para ver nuevos mensajes; esta vista no reanuda el agente.", "Cargando conversación guardada…",
        "Vista previa limitada: primeros 8 MiB del archivo, hasta 12.000 caracteres por mensaje.", "Copiar vista previa", "Aún no hay mensajes guardados.",
    ],
    "fr": [
        "Sous-agents (%{count})", "Sous-agents sans session parente (%{count})", "Sous-agent", "Inclure les sous-agents", "Masquer les sous-agents", "Actualiser",
        "Conversation enregistrée en lecture seule. Actualisez pour voir les nouveaux messages ; cette vue ne relance pas l’agent.", "Chargement de la conversation enregistrée…",
        "Aperçu limité : les 8 premiers Mio du fichier, jusqu’à 12 000 caractères par message.", "Copier l’aperçu", "Aucun message enregistré pour le moment.",
    ],
    "de": [
        "Unteragenten (%{count})", "Unteragenten ohne übergeordnete Sitzung (%{count})", "Unteragent", "Unteragenten einbeziehen", "Unteragenten ausblenden", "Aktualisieren",
        "Gespeicherter Verlauf, nur lesbar. Zum Anzeigen neuer Nachrichten aktualisieren; der Agent wird dadurch nicht fortgesetzt.", "Gespeicherten Verlauf laden…",
        "Begrenzte Vorschau: erste 8 MiB der Datei, bis zu 12.000 Zeichen pro Nachricht.", "Vorschau kopieren", "Noch keine gespeicherten Nachrichten.",
    ],
    "pt-BR": [
        "Subagentes (%{count})", "Subagentes sem sessão principal (%{count})", "Subagente", "Incluir subagentes", "Ocultar subagentes", "Atualizar",
        "Conversa salva somente para leitura. Atualize para ver novas mensagens; esta visualização não retoma o agente.", "Carregando conversa salva…",
        "Prévia limitada: primeiros 8 MiB do arquivo, até 12.000 caracteres por mensagem.", "Copiar prévia", "Ainda não há mensagens salvas.",
    ],
    "ru": [
        "Субагенты (%{count})", "Субагенты без родительской сессии (%{count})", "Субагент", "Включить субагентов", "Скрыть субагентов", "Обновить",
        "Сохранённый диалог только для чтения. Обновите, чтобы увидеть новые сообщения; агент при этом не возобновляет работу.", "Загрузка сохранённого диалога…",
        "Ограниченный просмотр: первые 8 МиБ файла, до 12 000 символов на сообщение.", "Копировать предпросмотр", "Сохранённых сообщений пока нет.",
    ],
    "it": [
        "Sottoagenti (%{count})", "Sottoagenti senza sessione principale (%{count})", "Sottoagente", "Includi sottoagenti", "Nascondi sottoagenti", "Aggiorna",
        "Conversazione salvata in sola lettura. Aggiorna per vedere nuovi messaggi; questa vista non riprende l’esecuzione dell’agente.", "Caricamento della conversazione salvata…",
        "Anteprima limitata: primi 8 MiB del file, fino a 12.000 caratteri per messaggio.", "Copia anteprima", "Nessun messaggio salvato ancora.",
    ],
}
for locale, translations in TRANSLATIONS.items():
    assert len(translations) == len(ENGLISH), locale
GLOSSARY = {locale: dict(zip(ENGLISH, values)) for locale, values in TRANSLATIONS.items()}
