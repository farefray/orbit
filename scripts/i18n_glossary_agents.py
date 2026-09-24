"""Read-only subagent inspector."""

ENGLISH = [
    "Queued", "Running", "Completed", "Steered", "Stopped", "Failed",
    "Started in background", "Unknown", "%{count} tools", "Inspect", "Agent inspector",
    "Latest reported tool data. Background updates and child conversation streaming are not available yet.",
    "Agent type", "Configuration", "Agent ID", "Run ID", "Turns", "Estimated cost",
    "Task outcome", "Task", "Last reported activity", "Assigned prompt", "Reported output",
    "Preview limited — copy for the full text", "Last reported: %{state}",
]

TRANSLATIONS = {
    "zh-CN": [
        "排队中", "运行中", "已完成", "已引导", "已停止", "失败", "已在后台启动", "未知",
        "%{count} 个工具", "查看", "子代理检查器",
        "最近上报的工具数据。暂不支持后台更新和子代理对话流。",
        "代理类型", "配置", "代理 ID", "运行 ID", "轮次", "预估费用", "任务结果", "任务",
        "最近上报的活动", "分配的提示词", "上报的输出", "预览受限 — 复制以获取全文", "最近上报：%{state}",
    ],
    "ja": [
        "待機中", "実行中", "完了", "誘導済み", "停止", "失敗", "バックグラウンドで開始", "不明",
        "%{count} ツール", "詳細", "エージェント詳細",
        "最後に報告されたツールデータです。バックグラウンド更新と子エージェントの会話ストリーミングにはまだ対応していません。",
        "エージェント種別", "設定", "エージェント ID", "実行 ID", "ターン", "推定費用", "タスクの結果", "タスク",
        "最後に報告された活動", "割り当てたプロンプト", "報告された出力", "プレビューは一部のみです — 全文はコピーしてください", "最終報告：%{state}",
    ],
    "ko": [
        "대기 중", "실행 중", "완료", "지시 전달됨", "중지됨", "실패", "백그라운드에서 시작됨", "알 수 없음",
        "도구 %{count}개", "살펴보기", "에이전트 검사기",
        "마지막으로 보고된 도구 데이터입니다. 백그라운드 업데이트와 하위 에이전트 대화 스트리밍은 아직 지원하지 않습니다.",
        "에이전트 유형", "설정", "에이전트 ID", "실행 ID", "턴", "예상 비용", "작업 결과", "작업",
        "마지막으로 보고된 활동", "할당된 프롬프트", "보고된 출력", "미리보기 제한 — 전체 텍스트를 복사하세요", "마지막 보고: %{state}",
    ],
    "es": [
        "En cola", "En ejecución", "Completado", "Redirigido", "Detenido", "Fallido", "Iniciado en segundo plano", "Desconocido",
        "%{count} herramientas", "Inspeccionar", "Inspector de agentes",
        "Últimos datos comunicados por la herramienta. Aún no hay actualizaciones en segundo plano ni transmisión de conversaciones de subagentes.",
        "Tipo de agente", "Configuración", "ID del agente", "ID de ejecución", "Turnos", "Coste estimado", "Resultado de la tarea", "Tarea",
        "Última actividad comunicada", "Instrucción asignada", "Salida comunicada", "Vista previa limitada — copia para obtener el texto completo", "Último estado comunicado: %{state}",
    ],
    "fr": [
        "En attente", "En cours", "Terminé", "Réorienté", "Arrêté", "Échec", "Démarré en arrière-plan", "Inconnu",
        "%{count} outils", "Inspecter", "Inspecteur d’agents",
        "Dernières données rapportées par l’outil. Les mises à jour en arrière-plan et la diffusion des conversations des sous-agents ne sont pas encore disponibles.",
        "Type d’agent", "Configuration", "ID de l’agent", "ID d’exécution", "Tours", "Coût estimé", "Résultat de la tâche", "Tâche",
        "Dernière activité rapportée", "Instruction assignée", "Sortie rapportée", "Aperçu limité — copiez pour obtenir le texte complet", "Dernier état rapporté : %{state}",
    ],
    "de": [
        "In Warteschlange", "Läuft", "Abgeschlossen", "Umgelenkt", "Gestoppt", "Fehlgeschlagen", "Im Hintergrund gestartet", "Unbekannt",
        "%{count} Werkzeuge", "Untersuchen", "Agenteninspektor",
        "Zuletzt gemeldete Werkzeugdaten. Hintergrundaktualisierungen und das Streaming von Unteragenten-Unterhaltungen sind noch nicht verfügbar.",
        "Agententyp", "Konfiguration", "Agenten-ID", "Lauf-ID", "Runden", "Geschätzte Kosten", "Aufgabenergebnis", "Aufgabe",
        "Zuletzt gemeldete Aktivität", "Zugewiesener Prompt", "Gemeldete Ausgabe", "Vorschau begrenzt — für den vollständigen Text kopieren", "Zuletzt gemeldet: %{state}",
    ],
    "pt-BR": [
        "Na fila", "Em execução", "Concluído", "Redirecionado", "Parado", "Falhou", "Iniciado em segundo plano", "Desconhecido",
        "%{count} ferramentas", "Inspecionar", "Inspetor de agentes",
        "Últimos dados informados pela ferramenta. Atualizações em segundo plano e transmissão de conversas de subagentes ainda não estão disponíveis.",
        "Tipo de agente", "Configuração", "ID do agente", "ID da execução", "Turnos", "Custo estimado", "Resultado da tarefa", "Tarefa",
        "Última atividade informada", "Prompt atribuído", "Saída informada", "Prévia limitada — copie para obter o texto completo", "Último estado informado: %{state}",
    ],
    "ru": [
        "В очереди", "Выполняется", "Завершено", "Перенаправлено", "Остановлено", "Ошибка", "Запущено в фоне", "Неизвестно",
        "%{count} инструментов", "Подробнее", "Инспектор агентов",
        "Последние данные инструмента. Фоновые обновления и трансляция диалогов субагентов пока недоступны.",
        "Тип агента", "Конфигурация", "ID агента", "ID запуска", "Ходы", "Оценка стоимости", "Результат задачи", "Задача",
        "Последняя известная активность", "Назначенный промпт", "Полученный вывод", "Предпросмотр ограничен — скопируйте полный текст", "Последний статус: %{state}",
    ],
    "it": [
        "In coda", "In esecuzione", "Completato", "Reindirizzato", "Arrestato", "Non riuscito", "Avviato in background", "Sconosciuto",
        "%{count} strumenti", "Ispeziona", "Ispettore agenti",
        "Ultimi dati riportati dallo strumento. Gli aggiornamenti in background e lo streaming delle conversazioni dei sottoagenti non sono ancora disponibili.",
        "Tipo di agente", "Configurazione", "ID agente", "ID esecuzione", "Turni", "Costo stimato", "Esito dell’attività", "Attività",
        "Ultima attività riportata", "Prompt assegnato", "Output riportato", "Anteprima limitata — copia per ottenere il testo completo", "Ultimo stato riportato: %{state}",
    ],
}

for locale, translations in TRANSLATIONS.items():
    assert len(translations) == len(ENGLISH), locale
GLOSSARY = {locale: dict(zip(ENGLISH, translations)) for locale, translations in TRANSLATIONS.items()}
