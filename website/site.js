// All user content is text, never HTML. The sole data source is public-profile.json.
export function safeUrl(value) {
  try {
    const url = new URL(value);
    return (
      typeof value === 'string' &&
      value.length <= 2048 &&
      !/[\u0000-\u0020]/u.test(value) &&
      ['https:', 'http:'].includes(url.protocol) &&
      !url.username &&
      !url.password
    );
  } catch {
    return false;
  }
}
function keys(value, allowed) {
  return (
    value &&
    typeof value === 'object' &&
    !Array.isArray(value) &&
    Object.keys(value).every((k) => allowed.includes(k))
  );
}
function text(value, max = 20000) {
  return typeof value === 'string' && Array.from(value).length <= max;
}
function links(values) {
  return (
    Array.isArray(values) &&
    values.length <= 20 &&
    values.every((x) => keys(x, ['label', 'url']) && text(x.label, 100) && safeUrl(x.url))
  );
}
const kinds = [
  'award',
  'competition',
  'paper',
  'project',
  'certificate',
  'open_source',
  'course_project',
  'research',
  'presentation',
  'other',
];
export function validatePublic(data) {
  if (
    !keys(data, ['schema_version', 'exported_at', 'profile', 'artifacts']) ||
    data.schema_version !== '1.0' ||
    !text(data.exported_at, 100) ||
    !Array.isArray(data.artifacts)
  )
    throw Error('Invalid public schema');
  if (
    data.profile !== null &&
    (!keys(data.profile, ['display_name', 'headline', 'about', 'links', 'public', 'revision']) ||
      data.profile.public !== true ||
      !text(data.profile.display_name, 100) ||
      !text(data.profile.headline, 300) ||
      !text(data.profile.about, 10000) ||
      !links(data.profile.links) ||
      data.profile.revision !== 0)
  )
    throw Error('Invalid public profile');
  for (const a of data.artifacts) {
    if (
      !keys(a, [
        'id',
        'title',
        'description',
        'kind',
        'date',
        'role',
        'organization',
        'skills',
        'technologies',
        'links',
        'public',
      ]) ||
      a.public !== true ||
      !/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/u.test(a.id) ||
      !kinds.includes(a.kind) ||
      !text(a.title, 300) ||
      !text(a.description) ||
      !text(a.role, 300) ||
      !text(a.organization, 300) ||
      !(a.date === null || /^\d{4}-\d{2}-\d{2}$/u.test(a.date)) ||
      !links(a.links) ||
      ![a.skills, a.technologies].every(
        (v) => Array.isArray(v) && v.length <= 50 && v.every((s) => text(s, 100)),
      )
    )
      throw Error('Invalid public artifact');
  }
  return data;
}
function element(tag, value, className) {
  const node = document.createElement(tag);
  if (value !== undefined) node.textContent = value;
  if (className) node.className = className;
  return node;
}
function anchor(link) {
  const a = element('a', link.label);
  a.href = link.url;
  a.target = '_blank';
  a.rel = 'noopener noreferrer';
  return a;
}
function card(a) {
  const node = element('article', undefined, 'card');
  node.append(
    element('p', [a.kind, a.date].filter(Boolean).join(' · '), 'eyebrow'),
    element('h3', a.title),
    element('p', a.description, 'preserve'),
    element('p', [a.role, a.organization].filter(Boolean).join(' · '), 'muted'),
  );
  const tags = element('div', undefined, 'tags');
  [...a.skills, ...a.technologies].forEach((t) => tags.append(element('span', t)));
  node.append(tags);
  const actions = element('div', undefined, 'actions');
  a.links.forEach((link) => actions.append(anchor(link)));
  const open = element('a', 'Open in Lumen');
  open.href = `lumen://artifact/${a.id}`;
  actions.append(open);
  node.append(actions);
  return node;
}
async function render() {
  try {
    const response = await fetch('./public-profile.json', { cache: 'no-store' });
    if (!response.ok) throw Error('No public data');
    const data = validatePublic(await response.json());
    if (data.profile) {
      document.getElementById('name').textContent = data.profile.display_name || '个人作品集';
      document.getElementById('headline').textContent = data.profile.headline;
      document.getElementById('bio').textContent = data.profile.about;
      data.profile.links.forEach((link) => document.querySelector('.contact').append(anchor(link)));
    } else document.getElementById('bio').textContent = '尚未公开个人介绍。';
    const groups = {
      projects: ['project', 'open_source', 'course_project'],
      research: ['research', 'paper'],
      competitions: ['competition'],
      awards: ['award', 'certificate'],
      portfolio: kinds,
    };
    for (const [id, kinds] of Object.entries(groups)) {
      const target = document.querySelector(`#${id} .grid`);
      const records = data.artifacts.filter((a) => kinds.includes(a.kind));
      if (!records.length) target.append(element('p', '暂无公开作品。', 'muted'));
      records.forEach((a) => target.append(card(a)));
    }
    document.getElementById('status').textContent =
      `公开作品 ${data.artifacts.length} · 导出时间 ${data.exported_at}`;
  } catch {
    document.getElementById('status').textContent =
      '公开资料不可用或格式不受支持。请通过本地 preview 服务器打开完整导出目录。';
  }
}
if (typeof document !== 'undefined' && document.getElementById('status')) void render();
