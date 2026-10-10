import { useState } from 'react';
import { Link } from 'react-router-dom';
import { Button } from '../../components/ui/button';
import { Card } from '../../components/ui/card';
import { Input } from '../../components/ui/input';
import { TaskForm } from '../../features/today/TaskForm';
import { duration, localDate, type TodayAction } from '../../features/today/model';
import { useToday } from '../../features/today/useToday';
import { todayText } from '../../features/today/text';
import { EventForm } from '../../features/calendar/EventForm';
import { useCalendar } from '../../features/calendar/useCalendar';
import {
  range,
  shift,
  onDay,
  timePosition,
  type CalendarView,
  type ActualInterval,
  type CalendarEvent,
} from '../../features/calendar/model';
import { calendarText as t } from '../../features/calendar/text';
function clock(value: string) {
  return new Date(value).toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}
export function CalendarPage() {
  const [day, setDay] = useState(localDate),
    [view, setView] = useState<CalendarView>('week'),
    [planned, setPlanned] = useState(true),
    [actual, setActual] = useState(true);
  const dates = range(day, view),
    calendar = useCalendar(dates.from, dates.until),
    today = useToday();
  const data = calendar.data,
    busy = today.busy,
    disabled = !data || today.browser;
  async function act(action: TodayAction) {
    const ok = await today.act(action);
    if (ok) await calendar.refresh();
    return ok;
  }
  const items = (date: string) => ({
    events: planned ? (data?.events.filter((event) => onDay(event, date)) ?? []) : [],
    tasks: planned ? (data?.tasks.filter((task) => task.due_date === date) ?? []) : [],
    actual: actual ? (data?.actual.filter((item) => item.local_date === date) ?? []) : [],
  });
  const actualLabel = (item: ActualInterval) =>
    `${t.real} · ${item.title} · ${duration(item.elapsed_ms / 1000)} · ${item.provenance === 'legacy_total' ? t.legacy : item.state === 'finished' ? t.finished : t.live}`;
  const eventLabel = (event: CalendarEvent) =>
    `${t.plan} · ${event.title} · ${t.types[event.event_type]} · ${event.event_date ? t.allDay : clock(event.starts_at!) + '–' + clock(event.ends_at!)}`;
  function details(date: string) {
    const entries = items(date);
    return (
      <section key={date} className="calendar-agenda-day" aria-label={date}>
        <h3>{date}</h3>
        <ul className="live-task-list">
          {entries.tasks.map((task) => (
            <li key={task.id}>
              <strong>
                {t.plan} · {task.title}
              </strong>
              <p className="secondary-text">
                {t.allDay} · {task.estimated_minutes} {todayText.minuteUnit} ·{' '}
                {todayText[task.lifecycle]}
              </p>
              <div className="task-actions">
                <TaskForm
                  task={task}
                  disabled={disabled || today.data?.session?.task_id === task.id}
                  busy={busy}
                  act={act}
                />
                <Link className="text-link" to="/focus">
                  {todayText.start}
                </Link>
              </div>
            </li>
          ))}
          {entries.events.map((event) => (
            <li key={event.id}>
              <strong>{eventLabel(event)}</strong>
              <div className="task-actions">
                <EventForm
                  event={event}
                  day={date}
                  tasks={data?.task_options ?? []}
                  disabled={disabled}
                  busy={busy}
                  act={act}
                />
                <Button
                  variant="ghost"
                  disabled={disabled || busy}
                  onClick={() =>
                    void act({
                      type: 'calendar_trash',
                      id: event.id,
                      revision: event.revision,
                      deleted: true,
                    })
                  }
                >
                  {t.remove}
                </Button>
              </div>
            </li>
          ))}
          {entries.actual.map((item) => (
            <li key={item.id} className="calendar-actual-row">
              <strong>{actualLabel(item)}</strong>
              <p className="secondary-text">
                {clock(item.starts_at)}–{clock(item.ends_at)}
              </p>
            </li>
          ))}
        </ul>
        {!entries.tasks.length && !entries.events.length && !entries.actual.length && (
          <p className="secondary-text">{t.empty}</p>
        )}
      </section>
    );
  }
  return (
    <>
      <div className="page-heading">
        <div>
          <p className="eyebrow">CALENDAR</p>
          <h1>{t.title}</h1>
          <p className="intro">{t.subtitle}</p>
        </div>
        <div className="task-actions">
          <EventForm
            day={day}
            tasks={data?.task_options ?? []}
            disabled={disabled}
            busy={busy}
            act={act}
          />
          <TaskForm disabled={disabled} busy={busy} act={act} />
        </div>
      </div>
      {calendar.loading && <p role="status">{t.load}</p>}
      {today.browser && <p className="preview-notice">{todayText.browser}</p>}
      {(calendar.error || today.error) && (
        <p role="alert" className="field-error">
          {calendar.error || today.error}
          <Button
            variant="secondary"
            disabled={busy}
            onClick={() => {
              void calendar.refresh();
              void today.refresh();
            }}
          >
            {t.refresh}
          </Button>
        </p>
      )}
      <Card className="calendar-card">
        <div className="calendar-toolbar">
          <div className="task-actions" role="group" aria-label={t.date}>
            <Button variant="secondary" onClick={() => setDay(shift(day, view, -1))}>
              {t.previous}
            </Button>
            <Button variant="secondary" onClick={() => setDay(localDate())}>
              {t.today}
            </Button>
            <Button variant="secondary" onClick={() => setDay(shift(day, view, 1))}>
              {t.next}
            </Button>
            <label className="sr-only" htmlFor="calendar-date">
              {t.date}
            </label>
            <Input
              id="calendar-date"
              type="date"
              value={day}
              onChange={(e) => {
                if (e.target.value) setDay(e.target.value);
              }}
            />
          </div>
          <div className="task-actions" role="group" aria-label={t.viewGroup}>
            {(Object.keys(t.views) as CalendarView[]).map((value) => (
              <Button
                key={value}
                variant={view === value ? 'default' : 'secondary'}
                aria-pressed={view === value}
                onClick={() => setView(value)}
              >
                {t.views[value]}
              </Button>
            ))}
          </div>
        </div>
        <div className="task-actions" role="group" aria-label={t.legend}>
          <Button variant="secondary" aria-pressed={planned} onClick={() => setPlanned(!planned)}>
            <span className="calendar-swatch planned" />
            {t.planned}
          </Button>
          <Button variant="secondary" aria-pressed={actual} onClick={() => setActual(!actual)}>
            <span className="calendar-swatch actual" />
            {t.actual}
          </Button>
        </div>
        <p className="secondary-text">
          {dates.from} — {dates.days.at(-1)} · {t.taskNote}
        </p>
        {view === 'month' ? (
          <>
            <div className="calendar-month-header">
              {t.weekdays.map((name) => (
                <span key={name}>{name}</span>
              ))}
            </div>
            <div className="calendar-month" aria-label={t.views.month}>
              {dates.days.map((date) => {
                const entry = items(date);
                return (
                  <div className={'calendar-cell' + (date === day ? ' selected' : '')} key={date}>
                    <button
                      className="calendar-day-button"
                      aria-label={t.dayOpen + ' ' + date}
                      aria-current={date === localDate() ? 'date' : undefined}
                      onClick={() => setDay(date)}
                    >
                      {Number(date.slice(-2))}
                    </button>
                    {[
                      ...entry.tasks.map((task) => ({
                        id: task.id,
                        label: t.plan + ' · ' + task.title,
                        real: false,
                      })),
                      ...entry.events.map((event) => ({
                        id: event.id,
                        label: t.plan + ' · ' + event.title,
                        real: false,
                      })),
                      ...entry.actual.map((item) => ({
                        id: item.id,
                        label: t.real + ' · ' + item.title,
                        real: true,
                      })),
                    ]
                      .slice(0, 3)
                      .map((item) => (
                        <div
                          key={item.id}
                          className={'calendar-chip ' + (item.real ? 'actual' : 'planned')}
                          title={item.label}
                        >
                          {item.label}
                        </div>
                      ))}
                    {entry.tasks.length + entry.events.length + entry.actual.length > 3 && (
                      <button
                        className="calendar-day-button"
                        aria-label={t.more + ' ' + date}
                        onClick={() => setDay(date)}
                      >
                        +{entry.tasks.length + entry.events.length + entry.actual.length - 3}
                      </button>
                    )}
                  </div>
                );
              })}
            </div>
            {details(day)}
          </>
        ) : view === 'agenda' ? (
          dates.days.map(details)
        ) : (
          <>
            <div className="calendar-time-scroll" tabIndex={0} aria-label={t.views[view]}>
              <div
                className="calendar-time-grid"
                style={{
                  gridTemplateColumns: `42px repeat(${dates.days.length}, minmax(${view === 'week' ? '80' : '180'}px,1fr))`,
                }}
              >
                <div className="calendar-hours">
                  <div className="calendar-column-heading">{t.allDay}</div>
                  <div className="calendar-hours-body">
                    {Array.from({ length: 24 }, (_, hour) => (
                      <span key={hour} style={{ top: (hour / 24) * 100 + '%' }}>
                        {String(hour).padStart(2, '0')}:00
                      </span>
                    ))}
                  </div>
                </div>
                {dates.days.map((date) => {
                  const entries = items(date);
                  return (
                    <div className="calendar-time-column" key={date}>
                      <div className="calendar-column-heading">
                        <button className="calendar-day-button" onClick={() => setDay(date)}>
                          {date.slice(5)}
                        </button>
                        <small>
                          {entries.tasks.length +
                            entries.events.filter((event) => event.event_date).length}{' '}
                          {t.allDay}
                        </small>
                      </div>
                      <div className="calendar-track">
                        {entries.events
                          .filter((event) => event.starts_at && event.ends_at)
                          .map((event) => {
                            const pos = timePosition(event.starts_at!, event.ends_at!, date);
                            return (
                              <button
                                key={event.id}
                                className="calendar-time-block planned"
                                style={{ top: pos.top + '%', height: `max(4px, ${pos.height}%)` }}
                                title={eventLabel(event)}
                                aria-label={eventLabel(event)}
                                onClick={() => setDay(date)}
                              >
                                <span>
                                  {t.plan} · {event.title}
                                </span>
                              </button>
                            );
                          })}
                        {entries.actual
                          .filter((item) => item.provenance !== 'legacy_total')
                          .map((item) => {
                            const pos = timePosition(item.starts_at, item.ends_at, date);
                            return (
                              <button
                                key={item.id}
                                className="calendar-time-block actual"
                                style={{ top: pos.top + '%', height: `max(4px, ${pos.height}%)` }}
                                title={actualLabel(item)}
                                aria-label={actualLabel(item)}
                                onClick={() => setDay(date)}
                              >
                                <span>
                                  {t.real} · {item.title}
                                </span>
                              </button>
                            );
                          })}
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
            <h2>{t.details}</h2>
            {dates.days.map(details)}
          </>
        )}
      </Card>
      <Card>
        <details>
          <summary>
            {t.trash} ({data?.trash.length ?? 0})
          </summary>
          {data?.trash.length ? (
            <ul className="live-task-list">
              {data.trash.map((event) => (
                <li key={event.id}>
                  <strong>{event.title}</strong>
                  <div className="task-actions">
                    <Button
                      variant="secondary"
                      disabled={disabled || busy}
                      onClick={() =>
                        void act({
                          type: 'calendar_trash',
                          id: event.id,
                          revision: event.revision,
                          deleted: false,
                        })
                      }
                    >
                      {t.restore}
                    </Button>
                  </div>
                </li>
              ))}
            </ul>
          ) : (
            <p className="secondary-text">{t.noTrash}</p>
          )}
        </details>
      </Card>
    </>
  );
}
