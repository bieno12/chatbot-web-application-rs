//get all conversations
var conversations = [];
let selected_conversation = undefined;
const aside_ul = document.querySelector("aside ul");
const chat = document.getElementById("chat");
const textarea = document.getElementById("message-textarea");
const sendButton = document.getElementById("send-button");

const search = document.querySelector("aside header input")
const newConversationButton = document.querySelector("aside header button");
const closeButton = document.querySelector("main header button");
function scrollToBottom() {
	const scrollOptions = {
	  top: chat.scrollHeight,
	  behavior: 'smooth'
	};
	chat.scrollTo(scrollOptions);
}

async function newConversation(name)
{
	const url = 'api/conversations/';
    const response = await fetch(url, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		  },
		body: JSON.stringify({name: name})
	});
    if (response.ok) {
        // The request was successful.
		console.log("new conversation was created");
        return await response.json();
    } else {
        // The request failed.
    }
}
async function getConversations() {
    const url = 'api/conversations/';

    const response = await fetch(url);

    if (response.ok) {
        // The request was successful.
		console.log("gettting conversations");

        return await response.json();
    } else {
        // The request failed.
    }
}
async function selectConversation(id)
{
	if (selected_conversation && selected_conversation.id == id)
		return ;
	selected_conversation = conversations.find((el)=> el.id == id);
	conversations.forEach(conversation => {
		if (conversation.id == selected_conversation.id)
			conversation.HTMLElement.setAttribute("class", "selected");
		else
		{
			conversation.HTMLElement.removeAttribute("class");
		}
	});
	chat.innerHTML = '';
	let messages = await getMessages(selected_conversation.id)
	console.log(messages);
	messages.forEach(message => {
		addMessageToChat(message);
	});
	scrollToBottom();
}
async function renameConversation(conversation, name)
{
	const url = `api/conversations/${conversation.id}`;
    const response = await fetch(url, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json',
		  },
		body: JSON.stringify({name: name})
	});
    if (response.ok) {
        // The request was successful.
		console.log("conversation was updated");
		conversation.HTMLElement.h2.innerHTML = name;
        return await response.json();
    } else {
        // The request failed.
    }
}

function editMode(conversation)
{
	conversation.HTMLElement.h2.style.display = 'none';
	const inputField = document.createElement("input");
	inputField.setAttribute('class', "text-input")
	inputField.value = conversation.HTMLElement.h2.innerHTML;
	const handler = ()=>{
		let newName = inputField.value.trim();
		if (newName != '')
			renameConversation(conversation, inputField.value);
		conversation.HTMLElement.h2.style.display = 'block';
		inputField.remove();
	}
	inputField.addEventListener("focusout", handler);
	inputField.addEventListener("submit", handler);
	inputField.addEventListener("keyup", (event) => {
		if (event.key === "Enter") {
		  handler();
		}
	  });
	conversation.HTMLElement.h2.parentElement.appendChild(inputField);
	inputField.focus();

}
function renderConversations()
{
	aside_ul.innerHTML = '';
	console.log((aside_ul));
	conversations = conversations.map(conversation => {
		const li = document.createElement("li")
		li.setAttribute("id", conversation.id);
		li.addEventListener("click", ()=>{selectConversation(conversation.id)})
		li.innerHTML += `
				<img  src="static/image/conversation_icon.png" alt="">
				<div>
					<h2>${conversation.name}</h2>
				</div>
		`;
		aside_ul.appendChild(li)
		conversation.HTMLElement = li;
		const h2 = li.querySelector("div h2");
		conversation.HTMLElement.h2 = h2;

		h2.addEventListener("dblclick", ()=>{editMode(conversation)})
		return conversation;
	});
}



async function getMessages(conversation_id)
{
	const url = `api/conversations/${conversation_id}/messages/`;
    const response = await fetch(url);
    if (response.ok) {
        // The request was successful.
		console.log("new conversation was created");
        return await response.json();
    } else {
        // The request failed.
    }
}
async function sendMessage(conversation_id, content)
{
	const url = `api/conversations/${conversation_id}/messages/`;
    const response = await fetch(url, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		  },
		body: JSON.stringify({"content": content})
	});
    if (response.ok) {
        // The request was successful.
		console.log("message sent in conversation " + conversation_id);
        return await response.json();
    } else {
        console.error(response)
    }
}
function addMessageToChat(message)
{
	const li = document.createElement("li");
	li.setAttribute("class", message.sender);
	chat.appendChild(li);
	if(message.sender == 'user')
	{
		li.innerHTML = `
			<div class="entete">
				<h3>${message.created_at}</h3>
				<h2>${message.sender}</h2>
				<span class="status blue"></span>
			</div>
			<div class="message">
				${message.content.replace(/(?:\r\n|\r|\n)/g, '<br>')}
			</div>
		`
	}
	else
	{
		li.innerHTML = `
			<div class="entete">
				<span class="status green"></span>
				<h2>${message.sender}</h2>
				<h3>${message.created_at}</h3>
			</div>
			<div class="message">
				${message.content}
			</div>
		`
	}
	scrollToBottom();
}
async function getBotResponse(conversation_id)
{
	const url = `api/conversations/${conversation_id}/messages/bot_response`;
    const response = await fetch(url);
    if (response.ok) {
        // The request was successful.
		console.log("bot responded " + conversation_id);
        return await response.json();
    } else {
        console.error(response)
    }
}

const sendMessageHandler = async ()=>{
	let content = textarea.value.trim();
	textarea.value = '';
	if (content != '')
	{
		var message = await sendMessage(selected_conversation.id, content);
		console.log(message)
		if(message)
			addMessageToChat(message);
		//get bot response
		let response = await getBotResponse(selected_conversation.id);
		if(response)
		addMessageToChat(response);
	}
}
sendButton.addEventListener("click", sendMessageHandler)
textarea.addEventListener("keydown", async (e)=>
{
	if(e.key != "Enter" || e.shiftKey) return;
	e.preventDefault();
	sendMessageHandler()

})
search.addEventListener("input", ()=>{
	query = search.value.toLowerCase();
	conversations.forEach(conversation => {
		if(conversation.name.toLowerCase().includes(query))
			conversation.HTMLElement.style.display = 'block';
		else
		conversation.HTMLElement.style.display = 'none';
	});
})
newConversationButton.addEventListener("click", async ()=>{
	const new_conv = await newConversation("New Conversation");
	conversations.push(new_conv);
	renderConversations()
	selectConversation(new_conv.id);
	
}
)
closeButton.addEventListener("click", async ()=>{
	const url = `api/conversations/${selected_conversation.id}`;
    const response = await fetch(url, {
		method: 'DELETE'
	});

    if (response.ok) {
        // The request was successful.
		console.log("deleted conversation");
		conversations = conversations.filter((conv)=> conv.id != selected_conversation.id);
		if (conversations.length == 0)
			conversations.push(await newConversation("New Conversation"));
		renderConversations();
		selectConversation(conversations[0].id);
        return await response.json();
    } else {
        // The request failed.
    }
})
async function entry()
{
	conversations = await getConversations()
	//create a conversation if none exists
	if (conversations.length == 0)
		conversations.push(await newConversation("New Conversation"));
	console.log(conversations);
	renderConversations();
	selectConversation(conversations[0].id);
}
entry()